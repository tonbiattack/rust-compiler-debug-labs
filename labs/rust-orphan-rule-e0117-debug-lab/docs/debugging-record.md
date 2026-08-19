# E0117 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`labels()`と`Display`実装 を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0117`を実行した。

## 最初に観測した事実

バグコミット`65b7b22`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| 表示形式の実装不足 | 同じ診断が残る | 定義と呼び出しを確認 | 直接原因ではない | 棄却 |
| ローカル型を使う必要性 | 修正後に受理される | 一箇所だけ変更 | `cargo check`成功 | 採用 |

## 確定した原因

外部トレイトと外部型の組合せには実装を追加できない。少なくとも一方を現在のクレートの型にする必要がある。 [1]

## 最小修正

`Vec<String>`への実装を削除し、ローカル型`Labels`へ`Display`を実装した。

## 回帰保証

`tests/labels.rs`が`labels()`の文字列結果を検証する。 修正後の`cargo check`と`cargo test`の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 65b7b22 && cargo check`でE0117を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0117の直接原因だけを扱う。より広いAPI設計や別の言語規則は対象外である。

## References

[1] [Rust Error Codes: E0117](https://doc.rust-lang.org/error_codes/E0117.html)
