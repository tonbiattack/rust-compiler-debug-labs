/// 基本待機時間へ上限を適用します。
pub fn capped_delay() -> u64 {
    let mut delay = 30;
    delay = delay.min(10);
    delay
}
