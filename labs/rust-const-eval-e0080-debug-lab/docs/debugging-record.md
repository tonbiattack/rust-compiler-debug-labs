# E0080 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`retry_delay()`と定数`RETRY_DELAY` を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0080`を実行した。

## 最初に観測した事実

バグコミット`3420faa`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0080]: evaluation of constant value failed` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| テストの期待値誤り | 診断が残る | 定義を確認 | 直接原因ではない | 棄却 |
| 定数型の表現範囲不足 | 最小変更で受理 | 対象箇所だけ変更 | `cargo check`成功 | 採用 |

## 確定した原因

u8で評価する200 + 100は整数オーバーフローとなり、定数値を評価できない。 [1]

## 最小修正

定数の型をu16へ変更し、300を表現可能にした。

## 回帰保証

`tests/retry_delay.rs`が300ミリ秒を検証する。 修正後の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 3420faa && cargo check`でE0080を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0080の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0080](https://doc.rust-lang.org/error_codes/E0080.html)
