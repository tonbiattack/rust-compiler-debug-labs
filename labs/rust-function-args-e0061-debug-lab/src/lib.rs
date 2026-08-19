/// 基本待機時間と試行回数から次の待機時間を返します。
pub fn retry_delay(base_seconds: u64, attempt: u32) -> u64 {
    base_seconds * u64::from(attempt + 1)
}

pub fn first_retry_delay() -> u64 {
    retry_delay(5, 1)
}
