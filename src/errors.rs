//! Unified error types with proper exit code mapping.
//!
//! Exit codes per constitution:
//! - 0: success
//! - 1: user input error
//! - 2: ffmpeg execution failure
//! - >2: internal error

/// Application-level error with explicit exit code mapping.
#[derive(Debug)]
pub enum FfError {
    UserInput(String),
    ExecutionFailure(String),
    /// Reserved for unexpected internal bugs. Exit code: 3
    #[allow(dead_code)]
    Internal(String),
}

impl std::fmt::Display for FfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfError::UserInput(msg) => write!(f, "Input error: {}", msg),
            FfError::ExecutionFailure(msg) => write!(f, "Execution error: {}", msg),
            FfError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for FfError {}

impl FfError {
    /// Maps this error to its designated exit code.
    pub fn exit_code(&self) -> i32 {
        match self {
            FfError::UserInput(_) => 1,
            FfError::ExecutionFailure(_) => 2,
            FfError::Internal(_) => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_codes() {
        assert_eq!(FfError::UserInput("bad input".into()).exit_code(), 1);
        assert_eq!(
            FfError::ExecutionFailure("ffmpeg failed".into()).exit_code(),
            2
        );
        assert_eq!(FfError::Internal("unexpected bug".into()).exit_code(), 3);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", FfError::UserInput("file not found".into())),
            "Input error: file not found"
        );
        assert_eq!(
            format!("{}", FfError::ExecutionFailure("not in PATH".into())),
            "Execution error: not in PATH"
        );
        assert_eq!(
            format!("{}", FfError::Internal("panic".into())),
            "Internal error: panic"
        );
    }
}
