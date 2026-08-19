# E0790 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`default_prefix()`と`Slug`トレイト を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0790`を実行した。

## 最初に観測した事実

バグコミット`d7facb4`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| 関数の戻り型推論不足 | 同じ診断が残る | 定義と呼び出しを確認 | 直接原因ではない | 棄却 |
| 実装型の選択不足 | 修正後に受理される | 一箇所だけ変更 | `cargo check`成功 | 採用 |

## 確定した原因

複数の型が同じトレイトを実装するため、レシーバを持たない関連関数はトレイト名だけから実装を一意に選べない。 [1]

## 最小修正

`Slug::prefix()`を`<News as Slug>::prefix()`へ置換し、既定実装を明示した。

## 回帰保証

`tests/prefix.rs`が`news`を返す方針を検証する。 修正後の`cargo check`と`cargo test`の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach d7facb4 && cargo check`でE0790を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0790の直接原因だけを扱う。より広いAPI設計や別の言語規則は対象外である。

## References

[1] [Rust Error Codes: E0790](https://doc.rust-lang.org/error_codes/E0790.html)
