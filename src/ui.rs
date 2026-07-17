use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, LineGauge, Paragraph, Sparkline, Wrap};
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
    let area = center(body, Constraint::Percentage(70), Constraint::Length(8));
    let [status, _, text_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
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
    frame.render_widget(typing_text(app), text_area);
}

fn draw_results(frame: &mut Frame, app: &App, body: Rect) {
    let t = app.theme();
    let area = center(body, Constraint::Length(40), Constraint::Length(12));
    let [stats_area, spark_area] =
        Layout::vertical([Constraint::Length(7), Constraint::Length(5)]).areas(area);
    let big = Style::new().fg(t.accent).add_modifier(ratatui::style::Modifier::BOLD);
    let dim = Style::new().fg(t.dim);
    let fg = Style::new().fg(t.fg);

    let lines = vec![
        Line::from(vec![
            Span::styled("wpm    ", dim),
            Span::styled(format!("{:.0}", app.wpm()), big),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("acc    ", dim),
            Span::styled(format!("{:.0}%", app.accuracy()), fg),
        ]),
        Line::from(vec![
            Span::styled("raw    ", dim),
            Span::styled(format!("{:.0}", app.raw_wpm()), fg),
        ]),
        Line::from(vec![
            Span::styled("errors ", dim),
            Span::styled(format!("{} words", app.word_errors()), fg),
        ]),
        Line::from(vec![
            Span::styled("time   ", dim),
            Span::styled(format!("{}s", app.duration.as_secs()), fg),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines).centered(), stats_area);

    if app.wpm_samples.len() > 1 {
        frame.render_widget(
            Sparkline::default()
                .data(&app.wpm_samples)
                .style(Style::new().fg(t.accent)),
            spark_area,
        );
    }
}

fn typing_text(app: &App) -> Paragraph<'_> {
    let t = app.theme();
    let cursor = app.typed.len();
    // current word bounds, for the bold highlight
    let word_start = app.target[..cursor]
        .iter()
        .rposition(|&c| c == ' ')
        .map_or(0, |i| i + 1);
    let word_end = app.target[cursor..]
        .iter()
        .position(|&c| c == ' ')
        .map_or(app.target.len(), |i| cursor + i);
    let spans: Vec<Span> = app
        .target
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let mut style = match app.typed.get(i) {
                Some(&typed) if typed == c => Style::new().fg(t.correct),
                // wrong char: show what the target expected, marked red
                Some(_) => Style::new().fg(t.wrong).add_modifier(ratatui::style::Modifier::CROSSED_OUT),
                None => Style::new().fg(t.dim),
            };
            if (word_start..word_end).contains(&i) {
                style = style.add_modifier(ratatui::style::Modifier::BOLD);
            }
            if i == cursor {
                style = style.fg(t.fg).add_modifier(ratatui::style::Modifier::UNDERLINED);
            }
            Span::styled(c.to_string(), style)
        })
        .collect();
    Paragraph::new(Line::from(spans)).wrap(Wrap { trim: true })
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
