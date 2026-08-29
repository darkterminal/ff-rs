use ffrs::{CommandBuilder, Parser, Tokenizer};

#[test]
fn test_command_generation_snapshot() {
    let test_cases = vec![
        (
            "convert video.mp4 to video.avi",
            vec!["-i", "video.mp4", "video.avi"],
        ),
        (
            "convert input.mov to output.mp4",
            vec!["-i", "input.mov", "output.mp4"],
        ),
        (
            "convert audio.wav to audio.mp3",
            vec!["-i", "audio.wav", "audio.mp3"],
        ),
    ];

    for (input, expected_args) in test_cases {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let mut parser = Parser::new(tokens);
        let intent = parser.parse().expect("Parsing should succeed");

        let cmd_builder = CommandBuilder::new();
        let result = cmd_builder
            .build_command(&intent)
            .expect("Command building should succeed");

        assert_eq!(result.program, "ffmpeg");
        assert_eq!(result.args, expected_args);
    }
}

#[test]
fn test_command_generation_snapshot_extended() {
    let input = "convert video.mp4 to .avi";

    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);
    let intent = parser.parse().expect("Parsing should succeed");

    let cmd_builder = CommandBuilder::new();
    let result = cmd_builder
        .build_command(&intent)
        .expect("Command building should succeed");

    assert_eq!(result.program, "ffmpeg");
    assert_eq!(result.args, vec!["-i", "video.mp4", "video.avi"]);
}
