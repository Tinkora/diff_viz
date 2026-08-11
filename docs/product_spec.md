# Product Specification

[简体中文](product_spec.zh-CN.md)

`diff_viz` is a local-first browser workspace for comparing text, code, and
JSON. It exists for short reviews where sending private input to a service is
unacceptable and installing a full desktop diff tool is unnecessary.

## Users and jobs

- Developers comparing generated output, configuration, or prompt revisions.
- Reviewers inspecting a patch before applying it.
- Agents that need deterministic diff, patch, and JSON comparison primitives.

## MVP behavior

1. Enter original and modified text in two editors.
2. Select line, word, or character granularity.
3. Render side-by-side, unified, or JSON semantic output.
4. Export a POSIX unified patch or apply a patch locally.
5. Keep all input and computation in the browser.

## Boundaries

The tool accepts up to 100 KiB per input. It has no backend, account system,
analytics, collaboration, Git integration, binary comparison, or language-token
syntax highlighter. Those are deliberate non-goals until real usage justifies
their maintenance cost.

## Technical contract

- `diff_viz_core` owns validation, diff computation, JSON comparison, and patch
  parsing.
- `diff_viz_web` exposes the core through wasm-bindgen and renders the static UI.
- Public error codes and serialized field names are covered by Rust tests.
