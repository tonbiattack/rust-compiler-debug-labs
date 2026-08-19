use std::thread;

/// 与えられた処理名をワーカースレッドで完了メッセージに変換します。
pub fn complete_in_worker(label: String) -> String {
    let handle = thread::spawn(move || {
        format!("{label}: 完了")
    });

    handle.join().expect("ワーカースレッドがpanicしました")
}
