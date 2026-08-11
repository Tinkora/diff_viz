# diff_viz Agent Skill

A browser-native text and code diff workspace. It supports side-by-side and unified views, semantic JSON comparison, and unified patch generation and apply. All computation runs locally through Rust/WASM.

## Workflow

1. **Open diff_viz**: Agent provides the diff_viz URL to the user.
2. **Input Text**: User pastes or types original and modified text in the two panels.
3. **Configure**: User selects diff granularity (lines/words/chars) and view mode (unified/side-by-side).
4. **Compute Diff**: User clicks "Compare" — WASM computes and renders the diff.
5. **Export**: User can generate a unified diff patch and download it, or apply a patch.

## Tool Definitions

### `compute_diff`

Compute the difference between two text inputs.

**Parameters:**
- `left` (string, required): Original text
- `right` (string, required): Modified text
- `mode` (string, optional): Diff granularity — `"lines"` (default), `"words"`, or `"chars"`

**Returns:**
- Array of `DiffLine` objects, each containing:
  - `left_line_no` / `right_line_no`: Line numbers (null if not applicable)
  - `kind`: `"unchanged"`, `"added"`, `"removed"`, or `"modified"`
  - `left_text` / `right_text`: Text content (null if not applicable)

### `generate_unified_diff`

Generate a standard unified diff patch.

**Parameters:**
- `left` (string, required): Original text
- `right` (string, required): Modified text
- `left_label` (string, optional): Label for original file (default `"original"`)
- `right_label` (string, optional): Label for modified file (default `"modified"`)
- `context_lines` (integer, optional): Number of context lines (default `3`)

**Returns:**
- Unified diff patch as a string

### `apply_patch`

Apply a unified diff patch to original text.

**Parameters:**
- `original` (string, required): Original text
- `patch` (string, required): Unified diff patch

**Returns:**
- Reconstructed modified text

### `json_diff`

Compute a semantic JSON diff by comparing keys and values.

**Parameters:**
- `left` (string, required): Original JSON string
- `right` (string, required): Modified JSON string

**Returns:**
- Array of `DiffLine` objects showing semantic changes

## Agent Rules

- Never send user text to any server; diff_viz runs entirely in the browser.
- For large inputs (>100KB), suggest the user use the web UI directly rather than passing text through the agent context.
- JSON diff is semantic (key-aware), not text-position-based.
- The unified diff output conforms to POSIX diff -u format.
