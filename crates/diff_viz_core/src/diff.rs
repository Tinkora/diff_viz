use crate::error::CoreError;
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};

/// Maximum size accepted for each text input.
pub const MAX_INPUT_BYTES: usize = 100 * 1024;

/// Diff granularity mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffMode {
    /// Compare whole lines (default).
    Lines,
    /// Compare individual words within lines.
    Words,
    /// Compare individual characters.
    Chars,
}

/// View display mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewMode {
    /// Classic unified diff output (like `diff -u`).
    Unified,
    /// Side-by-side comparison.
    SideBySide,
}

/// The kind of change represented by a diff line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffLineKind {
    /// Line is identical in both left and right.
    Unchanged,
    /// Line was added in the right (modified) side.
    Added,
    /// Line was removed from the left (original) side.
    Removed,
    /// Line was modified (present in both but changed).
    Modified,
}

/// A single line in a diff result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffLine {
    /// 1-based line number in the left (original) text, if applicable.
    pub left_line_no: Option<usize>,
    /// 1-based line number in the right (modified) text, if applicable.
    pub right_line_no: Option<usize>,
    /// The kind of change.
    pub kind: DiffLineKind,
    /// Text content from the left side, if applicable.
    pub left_text: Option<String>,
    /// Text content from the right side, if applicable.
    pub right_text: Option<String>,
}

impl DiffLine {
    fn new(
        left_line_no: Option<usize>,
        right_line_no: Option<usize>,
        kind: DiffLineKind,
        left_text: Option<String>,
        right_text: Option<String>,
    ) -> Self {
        Self {
            left_line_no,
            right_line_no,
            kind,
            left_text,
            right_text,
        }
    }
}

/// Compute a line-level diff between two strings.
///
/// Uses the `similar` crate for high-performance text diffing.
/// Returns a vector of `DiffLine` objects describing the changes.
pub fn compute_diff(left: &str, right: &str, mode: DiffMode) -> Result<Vec<DiffLine>, CoreError> {
    validate_inputs(left, right)?;
    if left.is_empty() && right.is_empty() {
        return Err(CoreError::EmptyInput);
    }

    match mode {
        DiffMode::Lines => compute_line_diff(left, right),
        DiffMode::Words => compute_word_diff(left, right),
        DiffMode::Chars => compute_char_diff(left, right),
    }
}

/// Compute diff at line granularity.
fn compute_line_diff(left: &str, right: &str) -> Result<Vec<DiffLine>, CoreError> {
    let diff = TextDiff::from_lines(left, right);
    let mut result = Vec::new();
    let mut left_line = 1usize;
    let mut right_line = 1usize;

    for change in diff.iter_all_changes() {
        let (kind, left_text, right_text, left_no, right_no) = match change.tag() {
            ChangeTag::Equal => {
                let ln = left_line;
                let rn = right_line;
                left_line += 1;
                right_line += 1;
                (
                    DiffLineKind::Unchanged,
                    Some(change.value().to_string()),
                    Some(change.value().to_string()),
                    Some(ln),
                    Some(rn),
                )
            }
            ChangeTag::Delete => {
                let ln = left_line;
                left_line += 1;
                (
                    DiffLineKind::Removed,
                    Some(change.value().to_string()),
                    None,
                    Some(ln),
                    None,
                )
            }
            ChangeTag::Insert => {
                let rn = right_line;
                right_line += 1;
                (
                    DiffLineKind::Added,
                    None,
                    Some(change.value().to_string()),
                    None,
                    Some(rn),
                )
            }
        };

        result.push(DiffLine::new(
            left_no, right_no, kind, left_text, right_text,
        ));
    }

    // Post-process: merge adjacent Delete+Insert pairs into Modified
    result = merge_modified_lines(result);

    Ok(result)
}

/// Merge adjacent Removed + Added pairs into Modified lines.
fn merge_modified_lines(lines: Vec<DiffLine>) -> Vec<DiffLine> {
    let mut result: Vec<DiffLine> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if lines[i].kind == DiffLineKind::Removed
            && i + 1 < lines.len()
            && lines[i + 1].kind == DiffLineKind::Added
        {
            // Merge into a single Modified line
            let removed = &lines[i];
            let added = &lines[i + 1];
            result.push(DiffLine::new(
                removed.left_line_no,
                added.right_line_no,
                DiffLineKind::Modified,
                removed.left_text.clone(),
                added.right_text.clone(),
            ));
            i += 2;
        } else {
            result.push(lines[i].clone());
            i += 1;
        }
    }

    result
}

/// Compute diff at word granularity by splitting each line into words.
fn compute_word_diff(left: &str, right: &str) -> Result<Vec<DiffLine>, CoreError> {
    // For word-level diff, we compare word-by-word within each line pair.
    // First get line-level diff to know which lines correspond.
    let line_diff = compute_line_diff(left, right)?;
    let mut result = Vec::new();

    for dl in &line_diff {
        match dl.kind {
            DiffLineKind::Unchanged => {
                result.push(dl.clone());
            }
            DiffLineKind::Removed => {
                result.push(dl.clone());
            }
            DiffLineKind::Added => {
                result.push(dl.clone());
            }
            DiffLineKind::Modified => {
                // For modified lines, do word-level diff within the pair
                let left_text = dl.left_text.as_deref().unwrap_or("");
                let right_text = dl.right_text.as_deref().unwrap_or("");

                let word_diff = TextDiff::from_words(left_text, right_text);

                // Build word-level line descriptions
                let mut left_parts = Vec::new();
                let mut right_parts = Vec::new();

                for change in word_diff.iter_all_changes() {
                    match change.tag() {
                        ChangeTag::Equal => {
                            left_parts.push(change.value().to_string());
                            right_parts.push(change.value().to_string());
                        }
                        ChangeTag::Delete => {
                            left_parts.push(format!("[-{}]", change.value()));
                        }
                        ChangeTag::Insert => {
                            right_parts.push(format!("[+{}]", change.value()));
                        }
                    }
                }

                result.push(DiffLine::new(
                    dl.left_line_no,
                    dl.right_line_no,
                    DiffLineKind::Modified,
                    Some(left_parts.join("")),
                    Some(right_parts.join("")),
                ));
            }
        }
    }

    Ok(result)
}

/// Compute diff at character granularity.
fn compute_char_diff(left: &str, right: &str) -> Result<Vec<DiffLine>, CoreError> {
    let diff = TextDiff::from_chars(left, right);
    let mut runs: Vec<(ChangeTag, String)> = Vec::new();

    for change in diff.iter_all_changes() {
        if let Some((tag, value)) = runs.last_mut()
            && *tag == change.tag()
        {
            value.push_str(change.value());
            continue;
        }
        runs.push((change.tag(), change.value().to_string()));
    }

    let mut result = Vec::new();
    let mut left_line = 1usize;
    let mut right_line = 1usize;

    let mut i = 0;
    while i < runs.len() {
        let (tag, value) = &runs[i];
        let (kind, left_text, right_text, consumed) = if *tag == ChangeTag::Delete
            && i + 1 < runs.len()
            && runs[i + 1].0 == ChangeTag::Insert
        {
            (
                DiffLineKind::Modified,
                Some(value.clone()),
                Some(runs[i + 1].1.clone()),
                2,
            )
        } else {
            match tag {
                ChangeTag::Equal => (
                    DiffLineKind::Unchanged,
                    Some(value.clone()),
                    Some(value.clone()),
                    1,
                ),
                ChangeTag::Delete => (DiffLineKind::Removed, Some(value.clone()), None, 1),
                ChangeTag::Insert => (DiffLineKind::Added, None, Some(value.clone()), 1),
            }
        };

        let left_no = left_text.as_ref().map(|_| left_line);
        let right_no = right_text.as_ref().map(|_| right_line);
        let left_consumed = left_text.as_deref().map(line_span).unwrap_or(0);
        let right_consumed = right_text.as_deref().map(line_span).unwrap_or(0);
        result.push(DiffLine::new(
            left_no, right_no, kind, left_text, right_text,
        ));
        left_line += left_consumed;
        right_line += right_consumed;
        i += consumed;
    }

    Ok(result)
}

fn line_span(text: &str) -> usize {
    text.matches('\n').count().max(1)
}

fn validate_inputs(left: &str, right: &str) -> Result<(), CoreError> {
    if left.len() > MAX_INPUT_BYTES || right.len() > MAX_INPUT_BYTES {
        return Err(CoreError::InputTooLarge);
    }
    Ok(())
}

/// Generate a unified diff patch string (POSIX `diff -u` format).
///
/// # Arguments
/// * `left` - Original text
/// * `right` - Modified text
/// * `left_label` - Label for the original file in the header
/// * `right_label` - Label for the modified file in the header
/// * `context_lines` - Number of context lines around each change
pub fn generate_unified_diff(
    left: &str,
    right: &str,
    left_label: &str,
    right_label: &str,
    context_lines: usize,
) -> Result<String, CoreError> {
    validate_inputs(left, right)?;
    if left.is_empty() && right.is_empty() {
        return Err(CoreError::EmptyInput);
    }

    let diff = TextDiff::from_lines(left, right);
    Ok(diff
        .unified_diff()
        .context_radius(context_lines)
        .header(left_label, right_label)
        .to_string())
}

/// Apply a unified diff patch to original text, returning the modified text.
///
/// Parses standard unified diff format and reconstructs the modified text.
pub fn apply_patch(original: &str, patch: &str) -> Result<String, CoreError> {
    if patch.is_empty() {
        return Err(CoreError::PatchApplyError("Patch is empty".to_string()));
    }
    validate_inputs(original, patch)?;

    let original_lines = split_patch_lines(original);
    let patch_records = patch
        .split_inclusive('\n')
        .map(|line| line.strip_suffix('\n').unwrap_or(line).to_string())
        .collect::<Vec<_>>();
    let mut cursor = 0usize;
    while cursor < patch_records.len()
        && (patch_records[cursor].starts_with("--- ") || patch_records[cursor].starts_with("+++ "))
    {
        cursor += 1;
    }

    let mut result = Vec::new();
    let mut original_index = 0usize;
    let mut hunk_count = 0usize;
    while cursor < patch_records.len() {
        let header = patch_records[cursor].as_str();
        if !header.starts_with("@@") {
            return Err(CoreError::PatchApplyError(format!(
                "Unexpected patch content: {header}"
            )));
        }
        let (old_start, old_count, new_count) = parse_hunk_header(header)?;
        let expected_index = if old_count == 0 {
            old_start
        } else {
            old_start.saturating_sub(1)
        };
        if expected_index < original_index || expected_index > original_lines.len() {
            return Err(CoreError::PatchApplyError(format!(
                "Hunk starts at line {}, expected {}",
                old_start,
                original_index + 1
            )));
        }
        result.extend(
            original_lines[original_index..expected_index]
                .iter()
                .cloned(),
        );
        original_index = expected_index;
        cursor += 1;
        let mut hunk_old = 0usize;
        let mut hunk_new = 0usize;
        let mut body: Vec<PatchRecord> = Vec::new();
        while cursor < patch_records.len() && !patch_records[cursor].starts_with("@@") {
            let line = &patch_records[cursor];
            if line.starts_with('\\') {
                if line != "\\ No newline at end of file" || body.is_empty() {
                    return Err(CoreError::PatchApplyError(format!(
                        "Invalid patch marker: {line}"
                    )));
                }
                if let Some(last) = body.last_mut() {
                    last.newline = false;
                }
                cursor += 1;
                continue;
            }
            let (kind, text) = line.split_at(1);
            if !matches!(kind, " " | "+" | "-") {
                return Err(CoreError::PatchApplyError(format!(
                    "Invalid hunk line: {line}"
                )));
            }
            let record = PatchRecord {
                kind: kind.as_bytes()[0] as char,
                text: text.to_string(),
                newline: true,
            };
            if record.kind != '+' {
                hunk_old += 1;
            }
            if record.kind != '-' {
                hunk_new += 1;
            }
            body.push(record);
            cursor += 1;
        }
        if body.is_empty() || hunk_old != old_count || hunk_new != new_count {
            return Err(CoreError::PatchApplyError(format!(
                "Hunk count mismatch: expected -{}, +{}, got -{}, +{}",
                old_count, new_count, hunk_old, hunk_new
            )));
        }
        for record in body {
            match record.kind {
                ' ' => {
                    let source = original_lines.get(original_index).ok_or_else(|| {
                        CoreError::PatchApplyError("Context exceeds original text".to_string())
                    })?;
                    if !same_patch_content(source, &record) {
                        return Err(CoreError::PatchApplyError(
                            "Patch context does not match original text".to_string(),
                        ));
                    }
                    result.push(source.clone());
                    original_index += 1;
                }
                '-' => {
                    let source = original_lines.get(original_index).ok_or_else(|| {
                        CoreError::PatchApplyError("Deletion exceeds original text".to_string())
                    })?;
                    if !same_patch_content(source, &record) {
                        return Err(CoreError::PatchApplyError(
                            "Patch deletion does not match original text".to_string(),
                        ));
                    }
                    original_index += 1;
                }
                '+' => result.push(record),
                _ => unreachable!(),
            }
        }
        hunk_count += 1;
    }

    if hunk_count == 0 {
        return Err(CoreError::PatchApplyError("Patch has no hunks".to_string()));
    }
    result.extend(original_lines.into_iter().skip(original_index));
    Ok(join_patch_lines(&result))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PatchRecord {
    kind: char,
    text: String,
    newline: bool,
}

fn split_patch_lines(text: &str) -> Vec<PatchRecord> {
    if text.is_empty() {
        return Vec::new();
    }
    text.split_inclusive('\n')
        .map(|line| PatchRecord {
            kind: ' ',
            text: line.strip_suffix('\n').unwrap_or(line).to_string(),
            newline: line.ends_with('\n'),
        })
        .collect()
}

fn same_patch_content(left: &PatchRecord, right: &PatchRecord) -> bool {
    left.text == right.text && (right.newline || left.newline == right.newline)
}

fn join_patch_lines(lines: &[PatchRecord]) -> String {
    let mut result = String::new();
    for line in lines {
        result.push_str(&line.text);
        if line.newline {
            result.push('\n');
        }
    }
    result
}

fn parse_hunk_header(header: &str) -> Result<(usize, usize, usize), CoreError> {
    let body = header
        .strip_prefix("@@")
        .and_then(|value| value.split("@@").next())
        .ok_or_else(|| CoreError::PatchApplyError(format!("Invalid hunk header: {header}")))?;
    let mut ranges = body.split_whitespace();
    let old = ranges
        .next()
        .and_then(|range| range.strip_prefix('-'))
        .ok_or_else(|| CoreError::PatchApplyError(format!("Invalid hunk header: {header}")))?;
    let new = ranges
        .next()
        .and_then(|range| range.strip_prefix('+'))
        .ok_or_else(|| CoreError::PatchApplyError(format!("Invalid hunk header: {header}")))?;
    let (old_start, old_count) = parse_range(old, header)?;
    let (_, new_count) = parse_range(new, header)?;
    Ok((old_start, old_count, new_count))
}

fn parse_range(range: &str, header: &str) -> Result<(usize, usize), CoreError> {
    let mut parts = range.split(',');
    let start = parts
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .ok_or_else(|| CoreError::PatchApplyError(format!("Invalid hunk header: {header}")))?;
    let count = match parts.next() {
        Some(value) => value
            .parse::<usize>()
            .map_err(|_| CoreError::PatchApplyError(format!("Invalid hunk header: {header}")))?,
        None => 1,
    };
    if parts.next().is_some() {
        return Err(CoreError::PatchApplyError(format!(
            "Invalid hunk header: {header}"
        )));
    }
    Ok((start, count))
}

/// Compute a semantic JSON diff by comparing keys and values rather than raw text positions.
///
/// Parses both inputs as JSON, then recursively compares the resulting values.
/// Returns diff lines describing structural changes.
pub fn json_diff(left: &str, right: &str) -> Result<Vec<DiffLine>, CoreError> {
    validate_inputs(left, right)?;
    let left_val: serde_json::Value = serde_json::from_str(left)
        .map_err(|e| CoreError::InvalidJson(format!("Left input is not valid JSON: {}", e)))?;
    let right_val: serde_json::Value = serde_json::from_str(right)
        .map_err(|e| CoreError::InvalidJson(format!("Right input is not valid JSON: {}", e)))?;

    let mut result = Vec::new();
    compare_json_values(&left_val, &right_val, "$", &mut result, &mut 1, &mut 1);
    Ok(result)
}

/// Recursively compare two JSON values, appending DiffLine entries.
fn compare_json_values(
    left: &serde_json::Value,
    right: &serde_json::Value,
    path: &str,
    result: &mut Vec<DiffLine>,
    left_line: &mut usize,
    right_line: &mut usize,
) {
    match (left, right) {
        (serde_json::Value::Object(lmap), serde_json::Value::Object(rmap)) => {
            // Collect all keys
            let mut all_keys: Vec<&String> = lmap.keys().chain(rmap.keys()).collect();
            all_keys.sort();
            all_keys.dedup();

            for key in all_keys {
                let child_path = if path == "$" {
                    format!("$.{}", key)
                } else {
                    format!("{}.{}", path, key)
                };

                match (lmap.get(key), rmap.get(key)) {
                    (Some(lv), Some(rv)) => {
                        if lv == rv {
                            let ll = *left_line;
                            let rl = *right_line;
                            *left_line += 1;
                            *right_line += 1;
                            result.push(DiffLine::new(
                                Some(ll),
                                Some(rl),
                                DiffLineKind::Unchanged,
                                Some(format!("{}: {}", &child_path, json_value_summary(lv))),
                                Some(format!("{}: {}", &child_path, json_value_summary(rv))),
                            ));
                        } else {
                            compare_json_values(lv, rv, &child_path, result, left_line, right_line);
                        }
                    }
                    (Some(lv), None) => {
                        let ll = *left_line;
                        *left_line += 1;
                        result.push(DiffLine::new(
                            Some(ll),
                            None,
                            DiffLineKind::Removed,
                            Some(format!("{}: {}", &child_path, json_value_summary(lv))),
                            None,
                        ));
                    }
                    (None, Some(rv)) => {
                        let rl = *right_line;
                        *right_line += 1;
                        result.push(DiffLine::new(
                            None,
                            Some(rl),
                            DiffLineKind::Added,
                            None,
                            Some(format!("{}: {}", &child_path, json_value_summary(rv))),
                        ));
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        (serde_json::Value::Array(larr), serde_json::Value::Array(rarr)) => {
            let max_len = larr.len().max(rarr.len());
            for i in 0..max_len {
                let child_path = format!("{}[{}]", path, i);
                match (larr.get(i), rarr.get(i)) {
                    (Some(lv), Some(rv)) => {
                        if lv == rv {
                            let ll = *left_line;
                            let rl = *right_line;
                            *left_line += 1;
                            *right_line += 1;
                            result.push(DiffLine::new(
                                Some(ll),
                                Some(rl),
                                DiffLineKind::Unchanged,
                                Some(format!("{}: {}", &child_path, json_value_summary(lv))),
                                Some(format!("{}: {}", &child_path, json_value_summary(rv))),
                            ));
                        } else {
                            compare_json_values(lv, rv, &child_path, result, left_line, right_line);
                        }
                    }
                    (Some(lv), None) => {
                        let ll = *left_line;
                        *left_line += 1;
                        result.push(DiffLine::new(
                            Some(ll),
                            None,
                            DiffLineKind::Removed,
                            Some(format!("{}: {}", &child_path, json_value_summary(lv))),
                            None,
                        ));
                    }
                    (None, Some(rv)) => {
                        let rl = *right_line;
                        *right_line += 1;
                        result.push(DiffLine::new(
                            None,
                            Some(rl),
                            DiffLineKind::Added,
                            None,
                            Some(format!("{}: {}", &child_path, json_value_summary(rv))),
                        ));
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        _ => {
            // Different types or scalars
            if left == right {
                let ll = *left_line;
                let rl = *right_line;
                *left_line += 1;
                *right_line += 1;
                result.push(DiffLine::new(
                    Some(ll),
                    Some(rl),
                    DiffLineKind::Unchanged,
                    Some(format!("{}: {}", path, json_value_summary(left))),
                    Some(format!("{}: {}", path, json_value_summary(right))),
                ));
            } else {
                let ll = *left_line;
                let rl = *right_line;
                *left_line += 1;
                *right_line += 1;
                result.push(DiffLine::new(
                    Some(ll),
                    Some(rl),
                    DiffLineKind::Modified,
                    Some(format!("{}: {}", path, json_value_summary(left))),
                    Some(format!("{}: {}", path, json_value_summary(right))),
                ));
            }
        }
    }
}

/// Create a concise summary string for a JSON value.
fn json_value_summary(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            if s.chars().count() > 60 {
                let prefix = s.chars().take(57).collect::<String>();
                format!("\"{}...\"", prefix)
            } else {
                format!("\"{}\"", s)
            }
        }
        serde_json::Value::Array(a) => {
            format!("Array[{}]", a.len())
        }
        serde_json::Value::Object(o) => {
            format!("Object{{{}}}", o.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_line_diff_identical() {
        let left = "hello\nworld\n";
        let right = "hello\nworld\n";
        let result = compute_diff(left, right, DiffMode::Lines).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].kind, DiffLineKind::Unchanged);
        assert_eq!(result[1].kind, DiffLineKind::Unchanged);
    }

    #[test]
    fn test_compute_line_diff_added() {
        let left = "hello\n";
        let right = "hello\nworld\n";
        let result = compute_diff(left, right, DiffMode::Lines).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].kind, DiffLineKind::Unchanged);
        assert_eq!(result[1].kind, DiffLineKind::Added);
        assert_eq!(result[1].right_line_no, Some(2));
    }

    #[test]
    fn test_compute_line_diff_removed() {
        let left = "hello\nworld\n";
        let right = "hello\n";
        let result = compute_diff(left, right, DiffMode::Lines).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].kind, DiffLineKind::Unchanged);
        assert_eq!(result[1].kind, DiffLineKind::Removed);
    }

    #[test]
    fn test_compute_line_diff_modified() {
        let left = "hello\nworld\n";
        let right = "hello\nearth\n";
        let result = compute_diff(left, right, DiffMode::Lines).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].kind, DiffLineKind::Unchanged);
        assert_eq!(result[1].kind, DiffLineKind::Modified);
        assert_eq!(result[1].left_text.as_deref(), Some("world\n"));
        assert_eq!(result[1].right_text.as_deref(), Some("earth\n"));
    }

    #[test]
    fn test_empty_input() {
        let result = compute_diff("", "", DiffMode::Lines);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CoreError::EmptyInput);
    }

    #[test]
    fn test_generate_unified_diff() {
        let left = "line1\nline2\nline3\n";
        let right = "line1\nline2 modified\nline3\n";
        let patch = generate_unified_diff(left, right, "a.txt", "b.txt", 3).unwrap();
        assert!(patch.contains("--- a.txt"));
        assert!(patch.contains("+++ b.txt"));
        assert!(patch.contains("@@"));
        assert!(patch.contains("line2 modified"));
    }

    #[test]
    fn test_apply_patch() {
        let original = "line1\nline2\nline3";
        let patch =
            "--- a.txt\n+++ b.txt\n@@ -1,3 +1,3 @@\n line1\n-line2\n+line2 modified\n line3\n";
        let result = apply_patch(original, patch).unwrap();
        assert_eq!(result, "line1\nline2 modified\nline3");
    }

    #[test]
    fn test_apply_patch_empty() {
        assert!(apply_patch("", "--- a\n+++ b\n@@ -1 +1 @@\n-a\n+b\n").is_err());
        assert!(apply_patch("hello", "").is_err());
    }

    #[test]
    fn test_json_diff_simple() {
        let left = r#"{"name": "Alice", "age": 30}"#;
        let right = r#"{"name": "Alice", "age": 31}"#;
        let result = json_diff(left, right).unwrap();
        assert!(!result.is_empty());
        // Should find unchanged name and modified age
        let modified: Vec<_> = result
            .iter()
            .filter(|d| d.kind == DiffLineKind::Modified)
            .collect();
        assert_eq!(modified.len(), 1);
    }

    #[test]
    fn test_json_diff_added_key() {
        let left = r#"{"name": "Alice"}"#;
        let right = r#"{"name": "Alice", "age": 30}"#;
        let result = json_diff(left, right).unwrap();
        let added: Vec<_> = result
            .iter()
            .filter(|d| d.kind == DiffLineKind::Added)
            .collect();
        assert_eq!(added.len(), 1);
    }

    #[test]
    fn test_json_diff_invalid() {
        let result = json_diff("not json", r#"{"a": 1}"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_word_diff() {
        let left = "hello beautiful world\n";
        let right = "hello cruel world\n";
        let result = compute_diff(left, right, DiffMode::Words).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_compute_char_diff() {
        let left = "abc";
        let right = "adc";
        let result = compute_diff(left, right, DiffMode::Chars).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_compute_char_diff_keeps_both_sides_for_replacement() {
        let result = compute_diff("before", "after", DiffMode::Chars).unwrap();
        let modified = result
            .iter()
            .find(|line| line.kind == DiffLineKind::Modified)
            .expect("replacement should be represented as a modified line");
        assert_eq!(modified.left_text.as_deref(), Some("be"));
        assert_eq!(modified.right_text.as_deref(), Some("a"));
    }

    #[test]
    fn test_generated_patch_round_trips_multiple_hunks_and_zero_context() {
        let left = "a\nb\nc\nd\ne\nf\n";
        let right = "A\nb\nc\nd\nE\nf\n";
        let patch = generate_unified_diff(left, right, "a.txt", "b.txt", 0).unwrap();
        assert_eq!(apply_patch(left, &patch).unwrap(), right);
        assert_eq!(
            patch.lines().filter(|line| line.starts_with("@@")).count(),
            2
        );
    }

    #[test]
    fn test_generated_patch_round_trips_missing_final_newline() {
        let left = "one\ntwo\n";
        let right = "one\ntwo";
        let patch = generate_unified_diff(left, right, "a", "b", 3).unwrap();
        assert!(patch.contains("\\ No newline at end of file"));
        assert_eq!(apply_patch(left, &patch).unwrap(), right);
    }

    #[test]
    fn test_apply_patch_rejects_context_mismatch_and_bad_counts() {
        let wrong_context = "--- a\n+++ b\n@@ -1,2 +1,2 @@\n wrong\n-two\n+TWO\n";
        assert!(matches!(
            apply_patch("one\ntwo", wrong_context),
            Err(CoreError::PatchApplyError(_))
        ));

        let wrong_count = "--- a\n+++ b\n@@ -1,3 +1,3 @@\n one\n-two\n+TWO\n";
        assert!(matches!(
            apply_patch("one\ntwo", wrong_count),
            Err(CoreError::PatchApplyError(_))
        ));
    }

    #[test]
    fn test_apply_patch_allows_empty_original_for_insert() {
        let patch = "--- a\n+++ b\n@@ -0,0 +1 @@\n+created\n";
        assert_eq!(apply_patch("", patch).unwrap(), "created\n");
    }

    #[test]
    fn test_generated_patch_round_trips_insert_at_start_and_delete_all() {
        let inserted = generate_unified_diff("b\n", "a\nb\n", "a", "b", 0).unwrap();
        assert_eq!(apply_patch("b\n", &inserted).unwrap(), "a\nb\n");

        let deleted = generate_unified_diff("a\nb\n", "", "a", "b", 0).unwrap();
        assert_eq!(apply_patch("a\nb\n", &deleted).unwrap(), "");
    }

    #[test]
    fn test_input_size_limit_has_stable_error_code() {
        let input = "x".repeat(100 * 1024 + 1);
        let error = compute_diff(&input, "x", DiffMode::Lines).unwrap_err();
        assert_eq!(error.code(), "INPUT_TOO_LARGE");
    }

    #[test]
    fn test_json_diff_truncates_unicode_without_panicking() {
        let value = "界".repeat(40);
        let result = json_diff(&format!(r#"{{"value":"{}"}}"#, value), r#"{"value":"x"}"#).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].kind, DiffLineKind::Modified);
    }
}
