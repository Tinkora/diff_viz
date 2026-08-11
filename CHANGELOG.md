# Changelog

## [0.1.0] - 2026-08-06

### Added
- `diff_viz_core`: Diff computation (lines/words/chars), unified diff generation, patch apply, JSON semantic diff
- `diff_viz_web`: WASM bridge, HTML editor with side-by-side and unified views
- Agent Skill definition (`skills/`)
- Landing page
- CI workflow (native test, clippy, WASM check, wasm-pack build)

### Features
- Line-level, word-level, and character-level diff granularity
- Side-by-side and unified diff view modes
- Color-coded diff output: green (added), red (removed), yellow (modified)
- Generate standard unified diff patches with configurable context lines
- Apply patches to reconstruct modified text
- JSON semantic diff with changed paths and values
- Paste from clipboard in both panels
- Export unified diff patch as file
- Line numbers on both sides
- Responsive dark interface with English and Chinese labels
- All processing in WASM via the `similar` crate
