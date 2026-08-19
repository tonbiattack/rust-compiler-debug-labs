# E0038 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、公開関数`render()`と`&dyn Snapshot` を最小境界にした。`cargo check`、`cargo test`、`rustc --explain E0038`を実行した。

## 最初に観測した事実

バグコミット`7ec7cbe`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0038]: the trait `Snapshot` cannot be made into an object` を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| 識別子メソッドの可視性 | 同じ診断が残る | 定義と呼び出しを確認 | 直接原因ではない | 棄却 |
| Selfを返すメソッドのdyn互換性 | 修正後に受理される | 一箇所だけ変更 | `cargo check`成功 | 採用 |

## 確定した原因

トレイトオブジェクトでは、具体的な戻り型を決められない`Self`を返すメソッドをそのまま動的ディスパッチ表へ置けない。 [1]

## 最小修正

`duplicate`に`where Self: Sized`を付け、トレイトオブジェクトから利用できない静的操作として隔離した。

## 回帰保証

`tests/render.rs`が`&dyn Snapshot`越しの識別子描画を検証する。 修正後の`cargo check`と`cargo test`の出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 7ec7cbe && cargo check`でE0038を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0038の直接原因だけを扱う。より広いAPI設計や別の言語規則は対象外である。

## References

[1] [Rust Error Codes: E0038](https://doc.rust-lang.org/error_codes/E0038.html)
