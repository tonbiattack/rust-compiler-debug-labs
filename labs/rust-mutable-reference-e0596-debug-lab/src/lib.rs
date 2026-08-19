/// ラベルへ完了接尾辞を追加します。
pub fn mark_complete(label: &mut String) {
    label.push_str("-complete");
}
