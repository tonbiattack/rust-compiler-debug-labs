unsafe fn raw_marker() -> &'static str {
    "trusted"
}

pub fn marker() -> &'static str {
    // raw_markerは外部状態やポインタを扱わず、固定文字列だけを返す。
    unsafe { raw_marker() }
}
