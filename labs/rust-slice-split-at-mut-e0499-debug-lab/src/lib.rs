/// スライスの先頭2要素を固定値で更新します。
pub fn overwrite_first_two(values: &mut [i32]) {
    assert!(values.len() >= 2, "少なくとも2要素が必要です");

    let (first_part, remaining) = values.split_at_mut(1);
    first_part[0] = 10;
    remaining[0] = 20;
}
