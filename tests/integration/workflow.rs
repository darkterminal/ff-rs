use ffrs::{CommandBuilder, Parser, Tokenizer};

#[test]
fn test_complete_workflow() {
    let command = "convert test_input.mp4 to test_output.avi";

    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);
    let intent = parser.parse().expect("Parsing should succeed");

    let cmd_builder = CommandBuilder::new();
    let ffmpeg_cmd = cmd_builder
        .build_command(&intent)
        .expect("Command building should succeed");

    assert_eq!(ffmpeg_cmd.program, "ffmpeg");
    assert!(ffmpeg_cmd.args.contains(&"test_input.mp4".to_string()));
    assert!(ffmpeg_cmd.args.contains(&"test_output.avi".to_string()));

    // Note: We're not actually executing the command in tests to avoid requiring ffmpeg
    // and creating actual files. The Runner functionality is tested separately.
    println!("Generated command: {}", ffmpeg_cmd.to_display_string());
}
