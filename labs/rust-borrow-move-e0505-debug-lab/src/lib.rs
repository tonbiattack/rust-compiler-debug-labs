/// ラベルの先頭語と完全なラベルを連結します。
pub fn decorate_label() -> String {
    let label = String::from("daily-report");
    let prefix = &label[..5];

    format!("{prefix}: {label}")
}
