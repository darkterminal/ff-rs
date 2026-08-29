use ffrs::{CommandBuilder, Parser, Tokenizer};

#[test]
fn test_deterministic_behavior() {
    let input_command = "convert video.mp4 to video.avi";

    let result1 = process_command(input_command);
    let result2 = process_command(input_command);
    let result3 = process_command(input_command);

    assert_eq!(result1, result2);
    assert_eq!(result2, result3);

    assert!(result1.starts_with("ffmpeg"));
    assert!(result1.contains("video.mp4"));
    assert!(result1.contains("video.avi"));
}

fn process_command(command: &str) -> String {
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);
    let intent = parser.parse().expect("Parsing should succeed");

    let cmd_builder = CommandBuilder::new();
    let ffmpeg_cmd = cmd_builder
        .build_command(&intent)
        .expect("Command building should succeed");

    ffmpeg_cmd.to_display_string()
}
