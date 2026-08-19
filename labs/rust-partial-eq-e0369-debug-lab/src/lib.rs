/// リリース対象の時間枠です。
#[derive(PartialEq)]
pub struct ReleaseWindow {
    pub start_hour: u8,
    pub end_hour: u8,
}

/// 二つの時間枠が同じかを返します。
pub fn same_window(left: ReleaseWindow, right: ReleaseWindow) -> bool {
    left == right
}
