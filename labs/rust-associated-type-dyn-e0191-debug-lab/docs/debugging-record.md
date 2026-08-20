# E0191 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`read()`と`&dyn Storage` を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0191`を実行した。

## 最初に観測した事実

バグコミット`b82342d`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0191]: the value of the associated type `Item` in `Storage` must be specified` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| 実装型が未指定 | 診断が残る | 定義と境界を確認 | 直接原因ではない | 棄却 |
| 関連型が未指定 | 一箇所の変更で受理 | 最小修正を適用 | `cargo check`成功 | 採用 |

## 確定した原因

トレイトオブジェクトでは、すべての関連型の具体値を指定する必要がある。 [1]

## 最小修正

`&dyn Storage`を`&dyn Storage<Item = u32>`へ変更した。

## 回帰保証

`tests/read.rs`がトレイトオブジェクト越しに`7`を読めることを検証する。 修正後の`cargo check`と`cargo test`の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach b82342d && cargo check`でE0191を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0191の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0191](https://doc.rust-lang.org/error_codes/E0191.html)
