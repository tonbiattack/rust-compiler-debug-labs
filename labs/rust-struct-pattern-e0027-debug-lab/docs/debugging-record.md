# E0027 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、外部依存なしの`Task`型と公開関数`priority()`を境界にした。`cargo check`、`cargo test`、`rustc --explain E0027`を実行した。

## 最初に観測した事実

バグコミット`815a513`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0027]: pattern does not mention field \`title\``を出力した。出力は`observed-cargo-*-bug.txt`に保存した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| フィールド名の誤記 | `priority`自体が解決しない | `priority`を残して確認 | エラーは`title`の省略 | 棄却 |
| 構造体パターンの網羅性 | 残りを`..`で無視すれば受理 | `priority, ..`へ変更 | コンパイル成功 | 採用 |

## 確定した原因

構造体パターンは全フィールドを明示するか、`..`で残りを無視する必要がある。[1]

## 最小修正

`Task { priority }`を`Task { priority, .. }`へ変更した。利用しない`title`を意図的に無視する以外の変更はしていない。

## 回帰保証

`tests/priority.rs`が`priority()`の戻り値`3`を検証する。修正後の`cargo check`と`cargo test`の成功出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 815a513 && cargo check`で失敗を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

本件はパターンの網羅性だけを扱う。フィールドを取り出すことで起こる所有権・借用上の設計は別題材である。

## References

[1] [Rust Error Codes: E0027](https://doc.rust-lang.org/error_codes/E0027.html)
