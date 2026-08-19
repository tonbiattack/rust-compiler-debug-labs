# E0407 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開トレイト`Renderer`、`Task`、統合テストを最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0407`を実行した。

## 最初に観測した事実

バグコミット`189b5c0`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0407]: method \`display_name\` is not a member of trait \`Renderer\``を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| `Renderer`のメソッド名の誤記 | `display_name`を宣言すれば受理 | トレイト宣言を確認 | 契約は`render`のみ | 棄却 |
| 型固有メソッドの配置誤り | 固有`impl`へ移せば受理 | `impl Task`へ移動 | コンパイル成功 | 採用 |

## 確定した原因

`impl Renderer for Task`には`Renderer`が定義するメソッドしか置けない。トレイトにないメソッドは固有`impl`へ分離できる。[1]

## 最小修正

`display_name`をトレイト実装から削除し、`impl Task`で`pub fn display_name`として定義した。`Renderer`の契約は変更していない。

## 回帰保証

`tests/renderer.rs`は`render()`と`display_name()`の結果を検証する。修正後の`cargo check`と`cargo test`は成功し、出力を`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 189b5c0 && cargo check`でE0407を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

共通契約として`display_name`を全実装型に要求したい場合は、トレイト宣言を変える別の設計がある。本ラボでは既存契約を増やさず、型固有APIとして分離する。

## References

[1] [Rust Error Codes: E0407](https://doc.rust-lang.org/error_codes/E0407.html)
