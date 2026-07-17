use std::time::{Duration, Instant};

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::theme::{Theme, THEMES};
use crate::words;

const WORD_CHUNK: usize = 50;
pub const DURATIONS: [u64; 4] = [15, 30, 60, 120];

#[derive(PartialEq)]
pub enum Phase {
    Idle,    // waiting for first keystroke
    Running, // clock ticking
    Done,    // results screen
}

pub struct App {
    pub running: bool,
    pub phase: Phase,
    /// target text: words joined by single spaces
    pub target: Vec<char>,
    /// what the user has typed so far
    pub typed: Vec<char>,
    pub theme_idx: usize,
    /// all keystrokes vs correct ones, for accuracy (backspace doesn't erase mistakes)
    pub keystrokes: u32,
    pub correct_keystrokes: u32,
    pub duration: Duration,
    started_at: Option<Instant>,
    /// frozen at test end so results don't drift
    final_elapsed: Duration,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            phase: Phase::Idle,
            target: words::random_words(WORD_CHUNK).join(" ").chars().collect(),
            typed: Vec::new(),
            theme_idx: 0,
            keystrokes: 0,
            correct_keystrokes: 0,
            duration: Duration::from_secs(30),
            started_at: None,
            final_elapsed: Duration::ZERO,
        }
    }

    pub fn theme(&self) -> &'static Theme {
        &THEMES[self.theme_idx]
    }

    pub fn restart(&mut self) {
        self.target = words::random_words(WORD_CHUNK).join(" ").chars().collect();
        self.typed.clear();
        self.keystrokes = 0;
        self.correct_keystrokes = 0;
        self.phase = Phase::Idle;
        self.started_at = None;
        self.final_elapsed = Duration::ZERO;
    }

    pub fn elapsed(&self) -> Duration {
        match self.phase {
            Phase::Done => self.final_elapsed,
            _ => self.started_at.map_or(Duration::ZERO, |t| t.elapsed()),
        }
    }

    pub fn remaining_secs(&self) -> u64 {
        self.duration.saturating_sub(self.elapsed()).as_secs()
    }

    /// monkeytype-style: correct chars / 5, per minute
    pub fn wpm(&self) -> f64 {
        let mins = self.elapsed().as_secs_f64() / 60.0;
        if mins == 0.0 {
            return 0.0;
        }
        let correct = self
            .typed
            .iter()
            .zip(&self.target)
            .filter(|(a, b)| a == b)
            .count();
        correct as f64 / 5.0 / mins
    }

    /// all typed chars / 5, per minute
    pub fn raw_wpm(&self) -> f64 {
        let mins = self.elapsed().as_secs_f64() / 60.0;
        if mins == 0.0 {
            return 0.0;
        }
        self.keystrokes as f64 / 5.0 / mins
    }

    pub fn accuracy(&self) -> f64 {
        if self.keystrokes == 0 {
            return 100.0;
        }
        self.correct_keystrokes as f64 / self.keystrokes as f64 * 100.0
    }

    /// called every loop iteration: end the test, top up words
    pub fn tick(&mut self) {
        if self.phase == Phase::Running {
            if self.elapsed() >= self.duration {
                self.final_elapsed = self.duration;
                self.phase = Phase::Done;
            } else if self.typed.len() + WORD_CHUNK / 2 > self.target.len() {
                // fast typist nearing the end: extend the target
                let more = words::random_words(WORD_CHUNK).join(" ");
                self.target.push(' ');
                self.target.extend(more.chars());
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            if key.code == KeyCode::Char('t') {
                self.theme_idx = (self.theme_idx + 1) % THEMES.len();
            }
            return;
        }
        match key.code {
            KeyCode::Esc => self.running = false,
            KeyCode::Tab => self.restart(),
            // duration only settable before the clock starts
            KeyCode::Char(c @ '1'..='4') if self.phase == Phase::Idle => {
                self.duration = Duration::from_secs(DURATIONS[c as usize - '1' as usize]);
            }
            KeyCode::Backspace if self.phase == Phase::Running => {
                self.typed.pop();
            }
            KeyCode::Char(c) if self.phase != Phase::Done && self.typed.len() < self.target.len() => {
                if self.phase == Phase::Idle {
                    self.phase = Phase::Running;
                    self.started_at = Some(Instant::now());
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::KeyModifiers;

    fn press(app: &mut App, c: char) {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }

    #[test]
    fn tracks_accuracy_and_starts_clock() {
        let mut app = App::new();
        app.target = "ab".chars().collect();
        press(&mut app, 'a'); // correct
        press(&mut app, 'x'); // wrong
        assert!(app.phase == Phase::Running);
        assert_eq!(app.keystrokes, 2);
        assert_eq!(app.correct_keystrokes, 1);
        assert_eq!(app.accuracy(), 50.0);
        // target full: further chars ignored, no panic
        press(&mut app, 'y');
        assert_eq!(app.keystrokes, 2);
    }
}
