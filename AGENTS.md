# Repository Guide for AI Agents

## Project Overview

diff_viz is a browser-native text/code diff visualization tool. It provides side-by-side and unified diff views with syntax-aware highlighting, supports plain text, JSON, and code, and generates unified diff patches — all running in WebAssembly.

## Architecture

```
diff_viz/
├── crates/
│   ├── diff_viz_core/       # Diff algorithms, patch generation, JSON diff
│   └── diff_viz_web/        # WASM bridge + HTML editor
├── docs/                     # Specifications
├── skills/                   # Agent Skill definitions (MCP tools)
└── index.html               # Product landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/diff_viz_core/src/diff.rs` | Core diff computation, unified diff generation, patch apply, JSON diff |
| `crates/diff_viz_core/src/error.rs` | Stable error type with machine-readable codes |
| `crates/diff_viz_core/src/wasm.rs` | WASM bindings (4 JS exports) |
| `crates/diff_viz_web/src/lib.rs` | WASM bridge re-export layer |
| `crates/diff_viz_web/static/index.html` | Full-featured diff viewer UI |
| `skills/diff_viz.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |
| `docs/product_spec.zh-CN.md` | Product specification |

## Build & Test Commands

```bash
# Run all tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p diff_viz_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/diff_viz_web
```

## Design Principles

1. **Browser-first**: All diff computation and rendering happens in-browser via WASM + HTML/CSS
2. **Zero server**: No backend required; everything runs in the browser
3. **Syntax-aware**: JSON diff uses semantic key comparison, not raw text
4. **Privacy-first**: Input text never leaves the browser
5. **WASM-powered**: Leverages the `similar` crate for high-performance diff algorithms

## Error Codes (Stable Machine-Readable)

| Code | Meaning |
|------|---------|
| `EMPTY_INPUT` | One or both inputs are empty |
| `PARSE_ERROR` | Failed to parse input (e.g. invalid JSON) |
| `PATCH_APPLY_ERROR` | Failed to apply a unified diff patch |
| `INVALID_JSON` | JSON input is not valid JSON |

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
