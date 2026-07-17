use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Chart, Dataset, GraphType, LineGauge, Paragraph};
use ratatui::Frame;

use crate::app::{App, Phase, DURATIONS};

pub fn draw(frame: &mut Frame, app: &App) {
    let t = app.theme();
    frame.render_widget(Block::new().style(Style::new().bg(t.bg)), frame.area());

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    frame.render_widget(
        Line::from(vec![
            Span::styled(" monkey", Style::new().fg(t.dim)),
            Span::styled("tui", Style::new().fg(t.accent).add_modifier(Modifier::BOLD)),
        ]),
        header,
    );
    frame.render_widget(
        Line::from(format!("{} ", t.name))
            .style(Style::new().fg(t.dim))
            .right_aligned(),
        header,
    );

    match app.phase {
        Phase::Done => draw_results(frame, app, body),
        _ => draw_test(frame, app, body),
    }

    let keys: &[(&str, &str)] = match app.phase {
        Phase::Idle => &[("1-4", "time"), ("ctrl+t", "theme"), ("tab", "restart"), ("esc", "quit")],
        Phase::Running => &[("tab", "restart"), ("esc", "quit")],
        Phase::Done => &[("tab", "next test"), ("ctrl+t", "theme"), ("esc", "quit")],
    };
    let mut spans = Vec::new();
    for (i, (key, desc)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("   ", Style::new()));
        }
        spans.push(Span::styled(*key, Style::new().fg(t.fg)));
        spans.push(Span::styled(format!(" {desc}"), Style::new().fg(t.dim)));
    }
    frame.render_widget(Line::from(spans).centered(), footer);
}

fn draw_test(frame: &mut Frame, app: &App, body: Rect) {
    let t = app.theme();
    let area = center(body, Constraint::Percentage(60), Constraint::Length(8));
    let [status, _, text_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(2),
        Constraint::Length(5),
    ])
    .areas(area);

    match app.phase {
        Phase::Idle => {
            // duration selector: the active choice is the hero
            let mut spans = vec![Span::styled("time  ", Style::new().fg(t.dim))];
            for &secs in &DURATIONS {
                let style = if app.duration.as_secs() == secs {
                    Style::new().fg(t.accent).add_modifier(Modifier::BOLD)
                } else {
                    Style::new().fg(t.dim)
                };
                spans.push(Span::styled(format!("{secs}  "), style));
            }
            frame.render_widget(Line::from(spans), status);
        }
        _ => {
            let [counter, gauge] =
                Layout::horizontal([Constraint::Length(14), Constraint::Fill(1)]).areas(status);
            frame.render_widget(
                Line::from(vec![
                    Span::styled(
                        format!("{:>3} ", app.remaining_secs()),
                        Style::new().fg(t.accent).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(format!("{:.0} wpm", app.wpm()), Style::new().fg(t.dim)),
                ]),
                counter,
            );
            frame.render_widget(
                LineGauge::default()
                    .ratio(
                        (app.elapsed().as_secs_f64() / app.duration.as_secs_f64()).clamp(0.0, 1.0),
                    )
                    .label("")
                    .filled_style(Style::new().fg(t.accent))
                    .unfilled_style(Style::new().fg(t.dim)),
                gauge,
            );
        }
    }
    frame.render_widget(typing_text(app, text_area.width as usize), text_area);
}

/// monkeytype-style text: typed chars brighten to fg, wrong go red, the
/// caret is a solid accent block; hand-wrapped, 3 rows around the cursor
fn typing_text(app: &App, width: usize) -> Paragraph<'_> {
    let t = app.theme();
    let cursor = app.typed.len();
    let width = width.saturating_sub(2).max(10);

    let style_at = |i: usize| {
        if i == cursor {
            return Style::new().bg(t.accent).fg(t.bg);
        }
        match app.typed.get(i) {
            Some(&typed) if typed == app.target[i] => Style::new().fg(t.fg),
            // wrong char: show what the target expected, marked red
            Some(_) => Style::new().fg(t.wrong).add_modifier(Modifier::CROSSED_OUT),
            None => Style::new().fg(t.dim),
        }
    };

    let mut rows: Vec<Vec<Span>> = vec![Vec::new()];
    let mut row_width = 0;
    let mut cursor_row = 0;
    for (start, end) in app.word_ranges() {
        let word_len = end - start;
        if row_width > 0 && row_width + word_len > width {
            rows.push(Vec::new());
            row_width = 0;
        }
        if cursor >= start && cursor <= end {
            cursor_row = rows.len() - 1;
        }
        let row = rows.last_mut().unwrap();
        for i in start..end {
            row.push(Span::styled(app.target[i].to_string(), style_at(i)));
        }
        if end < app.target.len() {
            row.push(Span::styled(" ", style_at(end))); // the real, typeable space
            row.push(Span::raw(" ")); // visual-only gap
        }
        row_width += word_len + 2;
    }

    let first = cursor_row.saturating_sub(1);
    let mut lines = Vec::new();
    for spans in rows.into_iter().skip(first).take(3) {
        if !lines.is_empty() {
            lines.push(Line::default());
        }
        lines.push(Line::from(spans));
    }
    Paragraph::new(lines)
}

fn draw_results(frame: &mut Frame, app: &App, body: Rect) {
    let t = app.theme();
    let dim = Style::new().fg(t.dim);
    let fg = Style::new().fg(t.fg);

    let area = center(body, Constraint::Percentage(88), Constraint::Length(19));
    let [top, _, bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(2),
    ])
    .areas(area);
    let [left, _, chart_area] = Layout::horizontal([
        Constraint::Length(17),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(top);

    // hero numbers: wpm in accent, acc in fg
    let mut lines = vec![Line::from(Span::styled("wpm", dim))];
    for row in big_text(&format!("{:.0}", app.wpm())) {
        lines.push(Line::from(Span::styled(row, Style::new().fg(t.accent))));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("acc", dim)));
    for row in big_text(&format!("{:.0}%", app.accuracy())) {
        lines.push(Line::from(Span::styled(row, fg)));
    }
    frame.render_widget(Paragraph::new(lines), left);

    // bottom stat row, monkeytype-style
    let stats: [(&str, String); 5] = [
        ("test type", format!("time {} english", app.duration.as_secs())),
        ("raw", format!("{:.0}", app.raw_wpm())),
        (
            "characters",
            format!(
                "{}/{}",
                app.correct_keystrokes,
                app.keystrokes - app.correct_keystrokes
            ),
        ),
        ("consistency", format!("{:.0}%", app.consistency())),
        ("time", format!("{}s", app.duration.as_secs())),
    ];
    let cols = Layout::horizontal([Constraint::Fill(1); 5]).split(bottom);
    for (i, (label, value)) in stats.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled(*label, dim)),
                Line::from(Span::styled(value.clone(), fg)),
            ]),
            cols[i],
        );
    }

    if app.wpm_samples.len() > 1 {
        draw_chart(frame, app, chart_area);
    }
}

/// wpm (accent) + raw (dim) lines, error dots on the raw line where mistakes happened
fn draw_chart(frame: &mut Frame, app: &App, area: Rect) {
    let t = app.theme();
    let dim = Style::new().fg(t.dim);
    let as_points = |samples: &[u64]| -> Vec<(f64, f64)> {
        samples
            .iter()
            .enumerate()
            .map(|(i, &w)| ((i + 1) as f64, w as f64))
            .collect()
    };
    let wpm_points = as_points(&app.wpm_samples);
    let raw_points = as_points(&app.raw_samples);
    let error_points: Vec<(f64, f64)> = app
        .errors_per_sec
        .iter()
        .enumerate()
        .filter(|&(_, &e)| e > 0)
        .map(|(sec, _)| {
            let y = app.raw_samples.get(sec).copied().unwrap_or(0) as f64;
            ((sec + 1) as f64, y)
        })
        .collect();

    let max = raw_points
        .iter()
        .chain(&wpm_points)
        .map(|&(_, y)| y)
        .fold(1.0, f64::max);
    let n = wpm_points.len() as f64;

    let datasets = vec![
        Dataset::default()
            .name("raw")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(dim)
            .data(&raw_points),
        Dataset::default()
            .name("wpm")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::new().fg(t.accent))
            .data(&wpm_points),
        Dataset::default()
            .name("err")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::new().fg(t.wrong))
            .data(&error_points),
    ];

    frame.render_widget(
        Chart::new(datasets)
            .x_axis(
                Axis::default()
                    .bounds([1.0, n])
                    .labels([
                        Span::styled("1", dim),
                        Span::styled(format!("{:.0}", (n / 2.0).ceil()), dim),
                        Span::styled(format!("{n:.0}"), dim),
                    ])
                    .style(dim),
            )
            .y_axis(
                Axis::default()
                    .bounds([0.0, max * 1.1])
                    .labels([
                        Span::styled("0", dim),
                        Span::styled(format!("{:.0}", max / 2.0), dim),
                        Span::styled(format!("{max:.0}"), dim),
                    ])
                    .style(dim),
            ),
        area,
    );
}

/// 3x5 block-glyph font for hero numbers (digits and %)
fn glyph(c: char) -> [&'static str; 5] {
    match c {
        '0' => ["███", "█ █", "█ █", "█ █", "███"],
        '1' => ["  █", "  █", "  █", "  █", "  █"],
        '2' => ["███", "  █", "███", "█  ", "███"],
        '3' => ["███", "  █", "███", "  █", "███"],
        '4' => ["█ █", "█ █", "███", "  █", "  █"],
        '5' => ["███", "█  ", "███", "  █", "███"],
        '6' => ["███", "█  ", "███", "█ █", "███"],
        '7' => ["███", "  █", "  █", "  █", "  █"],
        '8' => ["███", "█ █", "███", "█ █", "███"],
        '9' => ["███", "█ █", "███", "  █", "███"],
        '%' => ["█ █", "  █", " █ ", "█  ", "█ █"],
        _ => ["   ", "   ", "   ", "   ", "   "],
    }
}

fn big_text(s: &str) -> [String; 5] {
    let mut rows: [String; 5] = Default::default();
    for c in s.chars() {
        let g = glyph(c);
        for (row, part) in rows.iter_mut().zip(g) {
            if !row.is_empty() {
                row.push(' ');
            }
            row.push_str(part);
        }
    }
    rows
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
