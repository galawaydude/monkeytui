use ratatui::style::Color;

pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    /// untyped text
    pub dim: Color,
    pub correct: Color,
    pub wrong: Color,
    pub accent: Color,
}

pub const THEMES: &[Theme] = &[Theme {
    name: "dark",
    bg: Color::Rgb(17, 17, 27),
    fg: Color::Rgb(205, 214, 244),
    dim: Color::Rgb(88, 91, 112),
    correct: Color::Rgb(166, 227, 161),
    wrong: Color::Rgb(243, 139, 168),
    accent: Color::Rgb(250, 179, 135),
}];
