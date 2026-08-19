/// 前後の空白を除き、小文字化したラベルを返します。
pub fn normalize_label(input: &str) -> String {
    input.trim().to_lowercase()
}
