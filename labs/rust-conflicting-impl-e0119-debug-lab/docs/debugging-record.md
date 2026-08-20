# E0119 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、Labelトレイトのblanket実装とTask実装を最小境界にした。

## 最初に観測した事実

バグコミット`3f03659`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0119]: conflicting implementations of trait Label for type Task`を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| Taskのフィールド定義が不正 | Taskを確認 | idは有効 | 直接原因ではない | 棄却 |
| blanket実装がTaskを含む | blanket implを削除 | cargo check成功 | 診断消失 | 採用 |

## 確定した原因

同じ型に対して重なり合うトレイト実装を定義できない。[1]

## 最小修正

Taskの固有ラベル実装と重複するblanket実装を削除した。

## 回帰保証

`tests/label.rs`がTaskの固有ラベルを検証する。成功出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 3f03659 && cargo check`でE0119を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0119の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0119](https://doc.rust-lang.org/error_codes/E0119.html)
