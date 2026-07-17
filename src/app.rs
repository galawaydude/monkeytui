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
    /// all keystrokes vs correct ones, for accuracy (backspace doesn't erase mistakes)
    pub keystrokes: u32,
    pub correct_keystrokes: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            target: words::random_words(WORD_COUNT).join(" ").chars().collect(),
            typed: Vec::new(),
            theme_idx: 0,
            keystrokes: 0,
            correct_keystrokes: 0,
        }
    }

    pub fn theme(&self) -> &'static Theme {
        &THEMES[self.theme_idx]
    }

    pub fn restart(&mut self) {
        self.target = words::random_words(WORD_COUNT).join(" ").chars().collect();
        self.typed.clear();
        self.keystrokes = 0;
        self.correct_keystrokes = 0;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.running = false,
            KeyCode::Tab => self.restart(),
            KeyCode::Backspace => {
                self.typed.pop();
            }
            KeyCode::Char(c) if self.typed.len() < self.target.len() => {
                self.typed.push(c);
                self.keystrokes += 1;
                if c == self.target[self.typed.len() - 1] {
                    self.correct_keystrokes += 1;
                }
            }
            _ => {}
        }
    }
}
