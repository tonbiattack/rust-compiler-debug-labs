/// 状態フィルターに応じたステータスを返します。
pub fn statuses(include_closed: bool) -> Box<dyn Iterator<Item = &'static str>> {
    if include_closed {
        Box::new(["open", "closed"].into_iter())
    } else {
        Box::new(std::iter::once("open"))
    }
}
