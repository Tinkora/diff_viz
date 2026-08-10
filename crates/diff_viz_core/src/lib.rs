mod diff;
mod error;

pub use diff::{
    DiffLine, DiffLineKind, DiffMode, MAX_INPUT_BYTES, ViewMode, apply_patch, compute_diff,
    generate_unified_diff, json_diff,
};
pub use error::CoreError;
