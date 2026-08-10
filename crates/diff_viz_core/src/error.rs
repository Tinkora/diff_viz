use thiserror::Error;

/// Stable error type for diff_viz operations.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("One or both inputs are empty")]
    EmptyInput,

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Failed to apply patch: {0}")]
    PatchApplyError(String),

    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    #[error("Input exceeds the 100 KiB limit")]
    InputTooLarge,
}

impl CoreError {
    /// Returns a stable machine error code for Web, CLI, and Agent consumers.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyInput => "EMPTY_INPUT",
            Self::ParseError(_) => "PARSE_ERROR",
            Self::PatchApplyError(_) => "PATCH_APPLY_ERROR",
            Self::InvalidJson(_) => "INVALID_JSON",
            Self::InputTooLarge => "INPUT_TOO_LARGE",
        }
    }
}
