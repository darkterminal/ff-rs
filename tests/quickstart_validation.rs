use ffrs::{CommandBuilder, Parser, Tokenizer};

#[test]
fn test_quickstart_scenarios() {
    // "convert video.mp4 to video.webm"
    let command = "convert video.mp4 to video.webm";
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let intent = parser.parse().expect("Parsing should succeed");
    let cmd_builder = CommandBuilder::new();
    let ffmpeg_cmd = cmd_builder
        .build_command(&intent)
        .expect("Command building should succeed");
    assert_eq!(ffmpeg_cmd.program, "ffmpeg");
    assert_eq!(ffmpeg_cmd.args, vec!["-i", "video.mp4", "video.webm"]);

    // "convert myvideo.mp4 to myvideo.avi"
    let command2 = "convert myvideo.mp4 to myvideo.avi";
    let mut tokenizer2 = Tokenizer::new(command2);
    let tokens2 = tokenizer2.tokenize();
    let mut parser2 = Parser::new(tokens2);
    let intent2 = parser2.parse().expect("Parsing should succeed");
    let ffmpeg_cmd2 = cmd_builder
        .build_command(&intent2)
        .expect("Command building should succeed");
    assert_eq!(ffmpeg_cmd2.args, vec!["-i", "myvideo.mp4", "myvideo.avi"]);

    // "convert video.mp4 to .avi"
    let command3 = "convert video.mp4 to .avi";
    let mut tokenizer3 = Tokenizer::new(command3);
    let tokens3 = tokenizer3.tokenize();
    let mut parser3 = Parser::new(tokens3);
    let intent3 = parser3.parse().expect("Parsing should succeed");
    let ffmpeg_cmd3 = cmd_builder
        .build_command(&intent3)
        .expect("Command building should succeed");
    assert_eq!(ffmpeg_cmd3.args, vec!["-i", "video.mp4", "video.avi"]);
}
