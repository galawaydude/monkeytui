use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyEventKind};

mod app;
mod theme;
mod ui;
mod words;

use app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    while app.running {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        // poll so the timer/wpm redraw even when the user isn't typing
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
        }
        app.tick();
    }

    ratatui::restore();
    Ok(())
}
