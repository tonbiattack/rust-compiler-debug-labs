# E0432 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、子モジュール`labels`と公開関数`status()`を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0432`を実行した。

## 最初に観測した事実

バグコミット`d29ee83`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0432]: unresolved import \`label\``を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| `format`が非公開 | 正しいモジュールでも失敗 | 定義を確認 | 関数は`pub` | 棄却 |
| モジュール名・相対パスの誤り | 正しい子モジュールを示せば受理 | `self::labels`へ変更 | コンパイル成功 | 採用 |

## 確定した原因

定義済みのモジュール名は`labels`なのに、`use label::format`と単数形を指定していた。E0432では名前と解決起点を確認する。[1]

## 最小修正

`use label::format`を`use self::labels::format`へ置換した。関数の実装や公開APIは変更していない。

## 回帰保証

`tests/status.rs`は`status()`が`status:ready`を返すことを検証する。修正後の`cargo check`と`cargo test`は成功し、出力を`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach d29ee83 && cargo check`でE0432を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

本件は現在のモジュールにある子モジュール名の解決に限定する。外部クレート、可視性、エディション差異が関わるimportは別途確認する。

## References

[1] [Rust Error Codes: E0432](https://doc.rust-lang.org/error_codes/E0432.html)
