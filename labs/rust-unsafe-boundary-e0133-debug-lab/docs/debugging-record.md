# E0133 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`marker()`とunsafe関数`raw_marker()` を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0133`を実行した。

## 最初に観測した事実

バグコミット`f940878`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0133]: call to unsafe function is unsafe and requires unsafe function or block` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| 戻り値の型不一致 | 診断が残る | 定義を確認 | 直接原因ではない | 棄却 |
| unsafe境界の欠如 | 最小変更で受理 | 対象箇所だけ変更 | `cargo check`成功 | 採用 |

## 確定した原因

unsafe関数の呼び出しは、安全性責任を明示するunsafe関数またはブロック内に限定する必要がある。 [1]

## 最小修正

固定文字列しか返さないことをコメントで示し、呼び出しを最小のunsafeブロックへ入れた。

## 回帰保証

`tests/marker.rs`がマーカー文字列を検証する。 修正後の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach f940878 && cargo check`でE0133を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0133の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0133](https://doc.rust-lang.org/error_codes/E0133.html)
