use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Chart, Dataset, GraphType, LineGauge, Paragraph};
use ratatui::Frame;

use crate::app::{App, Phase};

pub fn draw(frame: &mut Frame, app: &App) {
    let t = app.theme();
    frame.render_widget(Block::new().style(Style::new().bg(t.bg)), frame.area());

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    frame.render_widget(
        Line::from(" monkeytui ").style(Style::new().fg(t.accent).add_modifier(ratatui::style::Modifier::BOLD)),
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

    let help = match app.phase {
        Phase::Idle => " 1-4 time (15/30/60/120) · ctrl+t theme · tab restart · esc quit ",
        Phase::Running => " tab restart · esc quit ",
        Phase::Done => " tab next test · ctrl+t theme · esc quit ",
    };
    frame.render_widget(
        Line::from(help).style(Style::new().fg(t.dim)).centered(),
        footer,
    );
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
        Phase::Idle => frame.render_widget(
            Line::from(Span::styled(
                format!("{}s — start typing", app.duration.as_secs()),
                Style::new().fg(t.dim),
            )),
            status,
        ),
        _ => {
            let [counter, gauge] =
                Layout::horizontal([Constraint::Length(14), Constraint::Fill(1)]).areas(status);
            frame.render_widget(
                Line::from(vec![
                    Span::styled(format!("{:>3} ", app.remaining_secs()), Style::new().fg(t.accent)),
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

fn draw_results(frame: &mut Frame, app: &App, body: Rect) {
    let t = app.theme();
    let dim = Style::new().fg(t.dim);
    let fg = Style::new().fg(t.fg);
    let big = Style::new().fg(t.accent).add_modifier(Modifier::BOLD);

    let area = center(body, Constraint::Percentage(85), Constraint::Length(16));
    let [top, _, bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(2),
    ])
    .areas(area);
    let [left, chart_area] =
        Layout::horizontal([Constraint::Length(10), Constraint::Fill(1)]).areas(top);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled("wpm", dim)),
            Line::from(Span::styled(format!("{:.0}", app.wpm()), big)),
            Line::from(""),
            Line::from(Span::styled("acc", dim)),
            Line::from(Span::styled(format!("{:.0}%", app.accuracy()), big)),
        ]),
        left,
    );

    // bottom stat row, monkeytype-style
    let stats: [(&str, String); 5] = [
        ("test type", format!("time {}\nenglish", app.duration.as_secs())),
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
        let mut lines = vec![Line::from(Span::styled(*label, dim))];
        for part in value.split('\n') {
            lines.push(Line::from(Span::styled(part.to_string(), fg)));
        }
        frame.render_widget(Paragraph::new(lines), cols[i]);
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

/// monkeytype-style text: words wrapped by hand with double-space gaps,
/// blank line between rows, windowed to 3 rows around the cursor
fn typing_text(app: &App, width: usize) -> Paragraph<'_> {
    let t = app.theme();
    let cursor = app.typed.len();
    let width = width.saturating_sub(2).max(10);

    let style_at = |i: usize, current_word: bool| {
        let mut style = match app.typed.get(i) {
            Some(&typed) if typed == app.target[i] => Style::new().fg(t.correct),
            // wrong char: show what the target expected, marked red
            Some(_) => Style::new().fg(t.wrong).add_modifier(Modifier::CROSSED_OUT),
            None => Style::new().fg(t.dim),
        };
        if current_word {
            style = style.add_modifier(Modifier::BOLD);
        }
        if i == cursor {
            style = style.fg(t.fg).add_modifier(Modifier::UNDERLINED);
        }
        style
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
        let current_word = cursor >= start && cursor <= end;
        if current_word {
            cursor_row = rows.len() - 1;
        }
        let row = rows.last_mut().unwrap();
        for i in start..end {
            row.push(Span::styled(app.target[i].to_string(), style_at(i, current_word)));
        }
        if end < app.target.len() {
            row.push(Span::styled(" ", style_at(end, false))); // the real, typeable space
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

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
