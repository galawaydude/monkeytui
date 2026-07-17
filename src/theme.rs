use ratatui::style::Color;

pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    /// typed / live content
    pub fg: Color,
    /// labels, untyped text, everything by default
    pub dim: Color,
    pub wrong: Color,
    /// one hero element per screen
    pub accent: Color,
}

const fn rgb(hex: u32) -> Color {
    Color::Rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub const THEMES: &[Theme] = &[
    Theme {
        name: "catppuccin",
        bg: rgb(0x11111b),
        fg: rgb(0xcdd6f4),
        dim: rgb(0x585b70),
        wrong: rgb(0xf38ba8),
        accent: rgb(0xfab387),
    },
    Theme {
        name: "gruvbox",
        bg: rgb(0x282828),
        fg: rgb(0xebdbb2),
        dim: rgb(0x665c54),
        wrong: rgb(0xfb4934),
        accent: rgb(0xfe8019),
    },
    Theme {
        name: "dracula",
        bg: rgb(0x282a36),
        fg: rgb(0xf8f8f2),
        dim: rgb(0x6272a4),
        wrong: rgb(0xff5555),
        accent: rgb(0xbd93f9),
    },
    Theme {
        name: "nord",
        bg: rgb(0x2e3440),
        fg: rgb(0xeceff4),
        dim: rgb(0x4c566a),
        wrong: rgb(0xbf616a),
        accent: rgb(0x88c0d0),
    },
    Theme {
        name: "tokyonight",
        bg: rgb(0x1a1b26),
        fg: rgb(0xc0caf5),
        dim: rgb(0x565f89),
        wrong: rgb(0xf7768e),
        accent: rgb(0x7aa2f7),
    },
    Theme {
        name: "light",
        bg: rgb(0xfafafa),
        fg: rgb(0x383a42),
        dim: rgb(0xa0a1a7),
        wrong: rgb(0xe45649),
        accent: rgb(0xc18401),
    },
];
