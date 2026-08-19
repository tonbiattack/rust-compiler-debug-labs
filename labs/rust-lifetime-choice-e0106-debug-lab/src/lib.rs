/// 主ラベルが空でなければ主ラベルを、空なら既定ラベルを返します。
pub fn active_label<'a>(primary: &'a str, fallback: &'a str) -> &'a str {
    if primary.is_empty() {
        fallback
    } else {
        primary
    }
}
