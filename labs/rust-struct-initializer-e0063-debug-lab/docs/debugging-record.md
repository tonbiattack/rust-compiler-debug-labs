# E0063 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`daily()`と`Task`構造体リテラル を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0063`を実行した。

## 最初に観測した事実

バグコミット`2633227`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0063]: missing field `priority` in initializer of `Task`` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| フィールドの可視性 | 診断が残る | 定義と境界を確認 | 直接原因ではない | 棄却 |
| 構造体リテラルのフィールド省略 | 一箇所の変更で受理 | 最小修正を適用 | `cargo check`成功 | 採用 |

## 確定した原因

構造体リテラルでは各フィールドを一度ずつ指定する必要がある。 [1]

## 最小修正

`Task`の生成時に`priority: 1`を追加した。

## 回帰保証

`tests/daily.rs`が題名と優先度の両方を検証する。 修正後の`cargo check`と`cargo test`の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 2633227 && cargo check`でE0063を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0063の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0063](https://doc.rust-lang.org/error_codes/E0063.html)
