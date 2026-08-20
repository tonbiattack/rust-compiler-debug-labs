# E0525 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`twice()`とラベル生成クロージャ を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0525`を実行した。

## 最初に観測した事実

バグコミット`09b82a1`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| twiceの実装順序 | 診断が残る | 定義を確認 | 直接原因ではない | 棄却 |
| キャプチャしたStringの移動 | 最小変更で受理 | 対象箇所だけ変更 | `cargo check`成功 | 採用 |

## 確定した原因

Stringを移動して返すクロージャは一度しか呼べないFnOnceとなるが、twiceはFnを要求する。 [1]

## 最小修正

Stringをcloneして返し、クロージャが複数回呼べるFnを満たすようにした。

## 回帰保証

`tests/labels.rs`が2回ともtaskを生成することを検証する。 修正後の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 09b82a1 && cargo check`でE0525を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0525の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0525](https://doc.rust-lang.org/error_codes/E0525.html)
