use crate::command_builder::builder::FfmpegCommand;
use std::process::{Command, Stdio};

/// Runner for executing ffmpeg commands.
#[derive(Debug, Default)]
pub struct Runner;

/// Error types that can occur during command execution.
#[derive(Debug)]
pub enum ExecutionError {
    /// The command failed to execute
    CommandFailed(String),
    /// The command is invalid
    InvalidCommand(String),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            ExecutionError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl Runner {
    /// Creates a new command runner.
    pub fn new() -> Self {
        Self
    }

    /// Executes the given ffmpeg command.
    /// Streams stdout/stderr directly from ffmpeg (constitution requirement).
    pub fn execute(&self, cmd: &FfmpegCommand) -> Result<(), ExecutionError> {
        self.check_ffmpeg_availability()?;

        if cmd.args.is_empty() {
            return Err(ExecutionError::InvalidCommand(
                "Command has no arguments".to_string(),
            ));
        }

        let status = Command::new(&cmd.program)
            .args(&cmd.args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|e| {
                ExecutionError::CommandFailed(format!("Failed to execute command: {}", e))
            })?;

        if status.success() {
            Ok(())
        } else {
            Err(ExecutionError::CommandFailed(format!(
                "Command exited with status: {}",
                status
            )))
        }
    }

    /// Checks if ffmpeg is available in the system PATH.
    fn check_ffmpeg_availability(&self) -> Result<(), ExecutionError> {
        match Command::new("ffmpeg")
            .arg("-version")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(ExecutionError::CommandFailed(
                "ffmpeg is not available in PATH".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner_creation() {
        let runner = Runner::new();
        assert_eq!(format!("{:?}", runner), "Runner");
    }
}
