# E0207 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`default_count()`と`Defaults::value` を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0207`を実行した。

## 最初に観測した事実

バグコミット`4e1376a`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| Default実装の欠如 | 診断が残る | 定義と境界を確認 | 直接原因ではない | 棄却 |
| implパラメータの制約不足 | 一箇所の変更で受理 | 最小修正を適用 | `cargo check`成功 | 採用 |

## 確定した原因

`impl<T>`のTが自己型・実装トレイト・述語に現れず、コンパイラが実装を一意に扱えない。 [1]

## 最小修正

`T`を`impl`から外し、`value<T: Default>`のメソッド型パラメータへ移した。

## 回帰保証

`tests/default_count.rs`が`u32`の既定値`0`を検証する。 修正後の`cargo check`と`cargo test`の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 4e1376a && cargo check`でE0207を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0207の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0207](https://doc.rust-lang.org/error_codes/E0207.html)
