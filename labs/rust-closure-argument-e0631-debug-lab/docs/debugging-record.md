# E0631 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、`apply_to_count`が要求するクロージャ型を最小境界にした。

## 最初に観測した事実

バグコミット`4ba2c36`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0631]: type mismatch in closure arguments`を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| 戻り値の型が不正 | Fnの戻り値を確認 | u32で一致 | 直接原因ではない | 棄却 |
| 引数の型が不一致 | &strをu32へ変更 | cargo check成功 | 診断消失 | 採用 |

## 確定した原因

クロージャ引数の型は、受け取る高階関数が要求する型と一致しなければならない。[1]

## 最小修正

クロージャ引数を`u32`にして、入力値へ1を加えた。

## 回帰保証

`tests/count.rs`が3に処理を適用して4になることを検証する。成功出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 4ba2c36 && cargo check`でE0631を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0631の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0631](https://doc.rust-lang.org/error_codes/E0631.html)
