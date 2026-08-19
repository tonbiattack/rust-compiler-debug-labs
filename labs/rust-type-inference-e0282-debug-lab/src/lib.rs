/// 処理待ちキューが空かを返します。
pub fn is_queue_empty() -> bool {
    let queue: Vec<String> = Vec::new();

    queue.is_empty()
}
