//! diff_viz_web — WASM bridge for the browser-native diff visualization tool.
//!
//! Re-exports WASM bindings from diff_viz_core and provides the entry point
//! for wasm-bindgen compilation (cdylib crate type).

use diff_viz_core::{
    CoreError, DiffMode, apply_patch, compute_diff, generate_unified_diff, json_diff,
};
use wasm_bindgen::prelude::*;

/// Initialization hook called by the JS glue code.
/// Sets up console error panic hook for better debugging in the browser.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Compute a text diff and return serializable diff lines.
#[wasm_bindgen]
pub fn wasm_compute_diff(left: &str, right: &str, mode: &str) -> Result<JsValue, JsValue> {
    let mode = parse_diff_mode(mode)?;
    let lines = compute_diff(left, right, mode).map_err(core_error)?;
    to_js_value(&lines)
}

/// Generate a standard unified diff patch.
#[wasm_bindgen]
pub fn wasm_generate_unified_diff(
    left: &str,
    right: &str,
    left_label: &str,
    right_label: &str,
    context_lines: u32,
) -> Result<String, JsValue> {
    generate_unified_diff(left, right, left_label, right_label, context_lines as usize)
        .map_err(core_error)
}

/// Apply a unified diff patch after validating every hunk.
#[wasm_bindgen]
pub fn wasm_apply_patch(original: &str, patch: &str) -> Result<String, JsValue> {
    apply_patch(original, patch).map_err(core_error)
}

/// Compute a structural JSON diff.
#[wasm_bindgen]
pub fn wasm_json_diff(left: &str, right: &str) -> Result<JsValue, JsValue> {
    let lines = json_diff(left, right).map_err(core_error)?;
    to_js_value(&lines)
}

fn parse_diff_mode(mode: &str) -> Result<DiffMode, JsValue> {
    match mode {
        "lines" => Ok(DiffMode::Lines),
        "words" => Ok(DiffMode::Words),
        "chars" => Ok(DiffMode::Chars),
        _ => Err(js_error(
            "INVALID_MODE",
            "Diff mode must be lines, words, or chars",
        )),
    }
}

fn core_error(error: CoreError) -> JsValue {
    js_error(error.code(), &error.to_string())
}

fn js_error(code: &str, message: &str) -> JsValue {
    let object = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &object,
        &JsValue::from_str("code"),
        &JsValue::from_str(code),
    );
    let _ = js_sys::Reflect::set(
        &object,
        &JsValue::from_str("message"),
        &JsValue::from_str(message),
    );
    object.into()
}

fn to_js_value<T: serde::Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|error| js_error("SERIALIZATION_ERROR", &error.to_string()))
}
