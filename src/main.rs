use clap::Parser as ClapParser;
use std::io::{self, Write};

mod command_builder;
mod errors;
mod executor;
mod grammar;
mod intent;
mod utils;

use command_builder::CommandBuilder;
use errors::FfError;
use executor::runner::Runner;
use grammar::{Parser as GrammarParser, Tokenizer};

#[derive(ClapParser)]
#[command(name = "ff")]
#[command(about = "A CLI tool that translates plain English commands into ffmpeg commands")]
struct Cli {
    #[arg(value_parser)]
    command: Option<String>,

    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    #[arg(long, default_value_t = false)]
    dry_run: bool,

    #[arg(long)]
    output: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if args.interactive {
        run_interactive_mode(args.dry_run, args.output); // ← pass output
    } else if let Some(command) = args.command {
        run_direct_mode(&command, args.dry_run, args.output);
    } else {
        eprintln!("Error: No command provided. Use --help for usage information.");
        std::process::exit(1);
    }
}

fn run_interactive_mode(dry_run: bool, output: Option<String>) {
    // ← terima output
    println!("FF - Media Conversion Tool (Interactive Mode)");
    println!("Enter 'quit' or 'exit' to exit the program");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                    break;
                }
                if !input.is_empty() {
                    // clone() karena process_command consume Option<String>
                    match process_command(input, dry_run, output.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{}", e);
                            print_guidance(&e);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn run_direct_mode(command: &str, dry_run: bool, output: Option<String>) {
    match process_command(command, dry_run, output) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            print_guidance(&e);
            std::process::exit(e.exit_code());
        }
    }
}

fn print_guidance(err: &FfError) {
    match err {
        FfError::UserInput(msg) => {
            if msg.contains("not exist") || msg.contains("not found") {
                eprintln!(
                    "Guidance: Make sure the input file path is correct and the file exists."
                );
            } else if msg.contains("Unsupported format") {
                eprintln!("Guidance: Check supported formats: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF");
            } else {
                eprintln!("Guidance: Make sure your command follows the format 'convert <input> to <output>' or similar.");
                eprintln!("Example: 'convert video.mp4 to video.avi'");
            }
        }
        FfError::ExecutionFailure(msg) => {
            if msg.contains("not available") {
                eprintln!("Guidance: Install ffmpeg and ensure it is accessible in your PATH.");
            } else {
                eprintln!("Guidance: Check ffmpeg output above for details. Input file may be corrupted or format may be invalid.");
            }
        }
        FfError::Internal(_) => {
            eprintln!("Guidance: This is an unexpected bug. Please report it at https://github.com/darkterminal/ffrs/issues");
        }
    }
}

fn process_command(command: &str, dry_run: bool, output: Option<String>) -> Result<(), FfError> {
    // Step 1: Tokenize & Parse
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();

    let mut parser = GrammarParser::new(tokens);
    let intent = parser
        .parse()
        .map_err(|e| FfError::UserInput(e.to_string()))?;

    // Step 2: Fail Fast — Validate input file exists
    if !intent.input_path.exists() {
        return Err(FfError::UserInput(format!(
            "Input file does not exist: {}",
            intent.input_path.display()
        )));
    }
    if !intent.input_path.is_file() {
        return Err(FfError::UserInput(format!(
            "Input path is not a file: {}",
            intent.input_path.display()
        )));
    }

    // Step 3: Build ffmpeg command
    let cmd_builder = CommandBuilder::new();

    let ffmpeg_cmd = if let Some(output_dir) = output {
        let output_path = std::path::PathBuf::from(&output_dir).join(
            intent
                .output_path
                .file_name()
                .ok_or_else(|| FfError::UserInput("Invalid output path".to_string()))?,
        );
        cmd_builder
            .build_command_with_output_path(&intent, output_path)
            .map_err(|e| FfError::UserInput(e.to_string()))?
    } else {
        cmd_builder
            .build_command(&intent)
            .map_err(|e| FfError::UserInput(e.to_string()))?
    };

    // Step 4: Print the command (constitution requirement)
    println!("{}", ffmpeg_cmd.to_display_string());

    // Step 5: Execute (unless dry-run)
    if !dry_run {
        let runner = Runner::new();
        runner
            .execute(&ffmpeg_cmd)
            .map_err(|e| FfError::ExecutionFailure(e.to_string()))?;
    }

    Ok(())
}
