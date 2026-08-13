# diff_viz

[![CI](https://github.com/Tinkora/diff_viz/actions/workflows/ci.yml/badge.svg)](https://github.com/Tinkora/diff_viz/actions/workflows/ci.yml)
[![许可证：MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)

[![在 Ko-fi 上支持 Tinkora](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/tinkora)

`diff_viz` 是一个浏览器原生的文本和代码差异审查工作区。它支持并排或
统一补丁视图、JSON 语义差异、补丁生成与应用，所有计算均由 Rust/WASM
在浏览器本地完成，不会上传输入内容。

[打开在线工作区](https://tinkora.github.io/diff_viz/) · [English README](README.md)

## 功能

- 并排比较原始文本和修改后文本。
- 查看标准 `diff -u` 风格的统一差异。
- 按键和值比较 JSON，而不是比较原始位置。
- 在本地生成、导出和应用统一差异补丁。
- 支持按行、按词和按字符三种粒度。
- 静态页面无需后端，也可以离线运行。

## 快速开始

```bash
git clone https://github.com/Tinkora/diff_viz.git
cd diff_viz
wasm-pack build --target web --release --out-dir "$PWD/crates/diff_viz_web/static/pkg" crates/diff_viz_web
cd crates/diff_viz_web/static && python3 -m http.server 8080
```

然后访问 <http://localhost:8080>。

## 开发检查

```bash
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p diff_viz_web --target wasm32-unknown-unknown --locked
npm ci --ignore-scripts --prefix crates/diff_viz_web
npm run test:wasm-smoke:local --prefix crates/diff_viz_web
```

## 文档与社区

- [产品规格](docs/product_spec.md) · [产品规格（中文）](docs/product_spec.zh-CN.md)
- [贡献指南](CONTRIBUTING.zh-CN.md) · [English contributing guide](CONTRIBUTING.md)
- [安全策略](SECURITY.zh-CN.md) · [English security policy](SECURITY.md)
- [支持范围](SUPPORT.zh-CN.md) · [English support](SUPPORT.md)
- [更新日志](CHANGELOG.md)

## 许可证

MIT，版权归 [Tinkora contributors](https://github.com/Tinkora) 所有。
