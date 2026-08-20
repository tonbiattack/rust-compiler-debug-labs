# E0034 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、Taskに実装したShortTitleとLongTitleの同名メソッドを最小境界にした。

## 最初に観測した事実

バグコミット`63b4b60`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0034]: multiple applicable items in scope`を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| Taskがトレイト未実装 | implを確認 | 両方実装済み | 直接原因ではない | 棄却 |
| 同名候補が複数ある | 完全修飾構文に変更 | cargo check成功 | 診断消失 | 採用 |

## 確定した原因

同じ型に同名メソッド候補が複数あるため、コンパイラは呼び出すメソッドを決められない。[1]

## 最小修正

`<Task as ShortTitle>::title(&Task)`で選ぶトレイトを明示した。

## 回帰保証

`tests/title.rs`が短いタイトル`task`を検証する。成功出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 63b4b60 && cargo check`でE0034を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0034の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0034](https://doc.rust-lang.org/error_codes/E0034.html)
