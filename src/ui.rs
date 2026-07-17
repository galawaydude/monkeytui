use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

use crate::app::App;

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
        Line::from(" monkeytui ").style(Style::new().fg(t.accent).bold()),
        header,
    );

    let text_area = center(body, Constraint::Percentage(70), Constraint::Length(6));
    frame.render_widget(typing_text(app), text_area);

    frame.render_widget(
        Line::from(" tab restart · esc quit ")
            .style(Style::new().fg(t.dim))
            .centered(),
        footer,
    );
}

fn typing_text(app: &App) -> Paragraph<'_> {
    let t = app.theme();
    let cursor = app.typed.len();
    let spans: Vec<Span> = app
        .target
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let mut style = match app.typed.get(i) {
                Some(&typed) if typed == c => Style::new().fg(t.correct),
                // wrong char: show what the target expected, marked red
                Some(_) => Style::new().fg(t.wrong).crossed_out(),
                None => Style::new().fg(t.dim),
            };
            if i == cursor {
                style = style.fg(t.fg).underlined();
            }
            Span::styled(c.to_string(), style)
        })
        .collect();
    Paragraph::new(Line::from(spans)).wrap(ratatui::widgets::Wrap { trim: true })
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
