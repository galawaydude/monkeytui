use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::theme::{Theme, THEMES};
use crate::words;

const WORD_COUNT: usize = 50;

pub struct App {
    pub running: bool,
    /// target text: words joined by single spaces
    pub target: Vec<char>,
    /// what the user has typed so far
    pub typed: Vec<char>,
    pub theme_idx: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            target: words::random_words(WORD_COUNT).join(" ").chars().collect(),
            typed: Vec::new(),
            theme_idx: 0,
        }
    }

    pub fn theme(&self) -> &'static Theme {
        &THEMES[self.theme_idx]
    }

    pub fn restart(&mut self) {
        self.target = words::random_words(WORD_COUNT).join(" ").chars().collect();
        self.typed.clear();
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.running = false,
            KeyCode::Tab => self.restart(),
            _ => {}
        }
    }
}
