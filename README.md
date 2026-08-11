# diff_viz

[![CI](https://github.com/Tinkora/diff_viz/actions/workflows/ci.yml/badge.svg)](https://github.com/Tinkora/diff_viz/actions/workflows/ci.yml)
[![CodeQL](https://github.com/Tinkora/diff_viz/actions/workflows/codeql.yml/badge.svg)](https://github.com/Tinkora/diff_viz/actions/workflows/codeql.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

`diff_viz` is a browser-native text and code diff workspace. Compare two inputs
side by side or as a unified patch, inspect JSON changes semantically, and apply
or export patches. Rust/WASM performs every operation locally in the browser.

[Open the hosted workspace](https://tinkora.github.io/diff_viz/) · [中文说明](README.zh-CN.md)

## Features

- **Side-by-side diff**: Compare the original and modified inputs in parallel.
- **Unified diff view**: Inspect a standard `diff -u` style representation.
- **JSON semantic diff**: Compare keys and values instead of raw positions.
- **Patch generation and apply**: Export a unified patch or apply one locally.
- **Three granularities**: Lines, words, and characters.
- **Privacy first**: Input text stays in the browser and is never uploaded.
- **Zero backend**: The hosted page is static and can also be run offline.

## 🚀 Quick Start

```bash
# Clone
git clone https://github.com/Tinkora/diff_viz.git
cd diff_viz

# Build Web WASM
wasm-pack build --target web --release --out-dir "$PWD/crates/diff_viz_web/static/pkg" crates/diff_viz_web

# Launch the static page
cd crates/diff_viz_web/static && python3 -m http.server 8080
```

Open `http://localhost:8080` in your browser.

## 📂 Project Structure

| Component | Description | Status |
|-----------|-------------|--------|
| `diff_viz_core` | Diff algorithms, patch generation, JSON diff | ✅ |
| `diff_viz_web` | WASM bridge + HTML editor | ✅ |
| `skills/` | Agent Skill definition (MCP tools) | ✅ |

## 🔧 Development

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p diff_viz_web --target wasm32-unknown-unknown
```

## 📄 Docs

- [Product specification](docs/product_spec.md)
- [Maturity evidence](docs/MATURITY.md)
- [Release checklist](docs/RELEASE_CHECKLIST.md)

## 🤝 Community

- [Contributing](./CONTRIBUTING.md) · [中文贡献指南](./CONTRIBUTING.zh-CN.md)
- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [Security policy](./SECURITY.md) · [中文安全策略](./SECURITY.zh-CN.md)
- [Changelog](./CHANGELOG.md)

## 📜 License

MIT © [Tinkora contributors](https://github.com/Tinkora)
