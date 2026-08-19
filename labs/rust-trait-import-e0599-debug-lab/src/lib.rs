use std::io::{Cursor, Write};

/// メモリバッファへ状態文字列を書き込みます。
pub fn format_status() -> String {
    let mut buffer = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    writer
        .write_all(b"ready")
        .expect("メモリへの書き込みは成功する必要があります");

    String::from_utf8(buffer).expect("UTF-8文字列である必要があります")
}
