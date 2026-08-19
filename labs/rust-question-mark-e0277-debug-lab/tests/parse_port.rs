use rust_question_mark_e0277_debug_lab::parse_port;

#[test]
fn parses_a_valid_port_number() {
    assert_eq!(parse_port("8080"), Ok(8080));
}

#[test]
fn returns_a_readable_error_for_invalid_input() {
    let error = parse_port("eighty").expect_err("不正なポート番号は失敗する必要があります");

    assert!(error.contains("ポート番号"));
}
