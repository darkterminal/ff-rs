//! # FF - Media Conversion CLI Tool
//!
//! FF is a CLI tool that translates plain English commands into ffmpeg commands.
//! It provides both a library API and a command-line interface for media conversion tasks.
//!
//! ## Features
//!
//! - Translate plain English commands to ffmpeg commands
//! - Support for multiple media formats
//! - Interactive and direct command modes
//! - Dry-run functionality
//! - Deterministic and inspectable command generation

pub mod command_builder;
pub mod errors;
pub mod executor;
pub mod grammar;
pub mod intent;
pub mod utils;

pub use command_builder::{CommandBuilder, FfmpegCommand};
pub use errors::FfError;
pub use executor::runner::Runner;
pub use grammar::{Parser, Tokenizer};
pub use intent::types::{Intent, OperationType};
pub use utils::file_utils;
