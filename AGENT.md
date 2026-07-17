# Agent conventions

## Commit style

Conventional commits, imperative mood, lowercase summary, no trailing period:

```
<type>: <short imperative summary>
```

Types: `feat` (new capability), `fix` (bug fix), `refactor` (no behavior change),
`docs` (docs only), `chore` (build/tooling).

Examples:
- `feat: typing engine with per-char feedback`
- `fix: wpm divides by zero on instant restart`
- `docs: add install instructions`

One commit per milestone, pushed immediately after. Body optional — add only
when the summary can't carry the why.

## Project structure

```
src/
  main.rs    terminal setup + event loop
  app.rs     state machine (typing test logic)
  ui.rs      all rendering
  theme.rs   color palettes
  words.rs   embedded word list
```

Rules: rendering never mutates state; state never touches ratatui types
except in ui.rs; no new dependencies without need.
