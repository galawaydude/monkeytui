use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Style, Stylize};
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

    let text_area = center(body, Constraint::Percentage(70), Constraint::Length(3));
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
    let spans: Vec<Span> = app
        .target
        .iter()
        .map(|&c| Span::styled(c.to_string(), Style::new().fg(t.dim)))
        .collect();
    Paragraph::new(Line::from(spans)).wrap(ratatui::widgets::Wrap { trim: true })
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
