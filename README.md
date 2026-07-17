# monkeytui

An offline, terminal-based typing test in the spirit of
[monkeytype](https://monkeytype.com), built with [ratatui](https://ratatui.rs).

## Install

### From source (requires [Rust](https://rustup.rs))

```sh
cargo install --git https://github.com/galawaydude/monkeytui
```

or clone and build:

```sh
git clone https://github.com/galawaydude/monkeytui
cd monkeytui
cargo install --path .
```

Then run:

```sh
monkeytui
```

## Keys

| Key      | Action                                      |
|----------|---------------------------------------------|
| `1`-`4`  | test duration 15/30/60/120s (before typing) |
| `ctrl+t` | cycle theme                                 |
| `tab`    | restart / next test                         |
| `esc`    | quit                                        |

## Themes

catppuccin (default) · gruvbox · dracula · nord · tokyonight · light

## Structure

```
src/
  main.rs    terminal setup + event loop
  app.rs     test state & logic
  ui.rs      rendering
  theme.rs   color palettes
  words.rs   embedded word list (fully offline)
```
