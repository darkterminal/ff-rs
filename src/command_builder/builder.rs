use crate::intent::types::{Intent, OperationType};
use std::path::PathBuf;

/// Structured ffmpeg command — no string parsing needed downstream.
#[derive(Debug, Clone, PartialEq)]
pub struct FfmpegCommand {
    pub program: String,
    pub args: Vec<String>,
}

impl FfmpegCommand {
    /// Human-readable display string (for printing before execution).
    /// Quotes every argument for clarity, matching the CLI contract.
    pub fn to_display_string(&self) -> String {
        let mut parts = vec![self.program.clone()];
        parts.extend(self.args.iter().map(|a| format!("\"{}\"", a)));
        parts.join(" ")
    }
}

/// Command builder for converting intents into ffmpeg commands.
#[derive(Debug, Default)]
pub struct CommandBuilder;

impl CommandBuilder {
    /// Creates a new command builder.
    pub fn new() -> Self {
        Self
    }

    /// Builds an ffmpeg command from the given intent.
    pub fn build_command(
        &self,
        intent: &Intent,
    ) -> Result<FfmpegCommand, Box<dyn std::error::Error>> {
        self.build_command_with_output_path(intent, intent.output_path.clone())
    }

    /// Builds an ffmpeg command with a specific output path.
    pub fn build_command_with_output_path(
        &self,
        intent: &Intent,
        output_path: PathBuf,
    ) -> Result<FfmpegCommand, Box<dyn std::error::Error>> {
        let input_path = intent.input_path.to_string_lossy().to_string();
        let output_path = output_path.to_string_lossy().to_string();

        let args = match &intent.operation {
            OperationType::Convert => {
                vec!["-i".to_string(), input_path, output_path]
            }
            OperationType::Resize => {
                let width = intent
                    .parameters
                    .get("width")
                    .unwrap_or(&"1920".to_string())
                    .clone();
                let height = intent
                    .parameters
                    .get("height")
                    .unwrap_or(&"1080".to_string())
                    .clone();
                vec![
                    "-i".to_string(),
                    input_path,
                    "-vf".to_string(),
                    format!("scale={}:{}", width, height),
                    output_path,
                ]
            }
            OperationType::Transcode => {
                let video_codec = intent
                    .parameters
                    .get("vcodec")
                    .unwrap_or(&"libx264".to_string())
                    .clone();
                let audio_codec = intent
                    .parameters
                    .get("acodec")
                    .unwrap_or(&"aac".to_string())
                    .clone();
                vec![
                    "-i".to_string(),
                    input_path,
                    "-c:v".to_string(),
                    video_codec,
                    "-c:a".to_string(),
                    audio_codec,
                    output_path,
                ]
            }
            OperationType::ExtractAudio => {
                vec![
                    "-i".to_string(),
                    input_path,
                    "-q:a".to_string(),
                    "0".to_string(),
                    "-map".to_string(),
                    "a".to_string(),
                    output_path,
                ]
            }
        };

        Ok(FfmpegCommand {
            program: "ffmpeg".to_string(),
            args,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::types::{Intent, OperationType};
    use std::path::PathBuf;

    #[test]
    fn test_build_convert_command() {
        let builder = CommandBuilder::new();
        let intent = Intent {
            operation: OperationType::Convert,
            input_path: PathBuf::from("input.mp4"),
            output_path: PathBuf::from("output.avi"),
            parameters: std::collections::HashMap::new(),
        };
        let cmd = builder.build_command(&intent).unwrap();
        assert_eq!(cmd.program, "ffmpeg");
        assert_eq!(cmd.args, vec!["-i", "input.mp4", "output.avi"]);
        assert_eq!(
            cmd.to_display_string(),
            "ffmpeg \"-i\" \"input.mp4\" \"output.avi\""
        );
    }

    #[test]
    fn test_build_resize_command() {
        let builder = CommandBuilder::new();
        let mut params = std::collections::HashMap::new();
        params.insert("width".to_string(), "1280".to_string());
        params.insert("height".to_string(), "720".to_string());
        let intent = Intent {
            operation: OperationType::Resize,
            input_path: PathBuf::from("input.mp4"),
            output_path: PathBuf::from("output.mp4"),
            parameters: params,
        };
        let cmd = builder.build_command(&intent).unwrap();
        assert_eq!(cmd.program, "ffmpeg");
        assert_eq!(
            cmd.args,
            vec!["-i", "input.mp4", "-vf", "scale=1280:720", "output.mp4"]
        );
    }
}
