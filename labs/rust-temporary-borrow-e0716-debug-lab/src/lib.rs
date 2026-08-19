fn make_line() -> String {
    String::from("fast,safe")
}

fn first_segment(value: &str) -> &str {
    value.split(',').next().expect("区切り前の文字列")
}

/// 生成した行の最初の区切り要素の文字数を返します。
pub fn selected_length() -> usize {
    let line = make_line();
    let selected = first_segment(&line);

    selected.len()
}
