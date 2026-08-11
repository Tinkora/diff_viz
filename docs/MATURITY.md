# Maturity Evidence

[简体中文](MATURITY.zh-CN.md)

Current stage: **Alpha** (`v0.1.0`).

## Evidence available

- Core parsing and analysis are covered by outcome-focused Rust tests.
- The compiled WASM boundary is exercised in Chromium.
- The primary browser flow is verified at four viewport widths.
- Strict formatting, Clippy, dependency policy, and vulnerability checks exist.
- Public claims have been reduced to implemented and reproducible behavior.
- The repository is published under `Tinkora/diff_viz` with a clean project
  history and an immutable `v0.1.0` release.
- Hosted quality workflows, CodeQL, dependency policy, private vulnerability
  reporting, and the public Pages preview are required by repository rules.

## Required before Alpha

- Record at least one external or maintainer workflow using a real diff.

## Required before Beta

- Close at least one feedback loop with a user outside the implementation work.
- Demonstrate continued maintenance over multiple releases.
- Decide whether language-aware highlighting, redaction, or additional formats
  have enough measured demand to justify their maintenance cost.
