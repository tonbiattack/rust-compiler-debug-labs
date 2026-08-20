# E0276 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、`Render::render`の型境界を最小境界にした。

## 最初に観測した事実

バグコミット`b91ecad`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0276]: impl has stricter requirements than trait`を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| format!がDefaultを要求する | 型境界を確認 | Displayだけで整形可能 | 直接原因ではない | 棄却 |
| 実装が境界を追加した | Defaultを削除 | cargo check成功 | 診断消失 | 採用 |

## 確定した原因

トレイト実装はトレイト定義より厳しい要件をメソッドへ追加できない。[1]

## 最小修正

実装側の`Default`境界を削除し、トレイト定義と同じDisplay境界へ揃えた。

## 回帰保証

`tests/render.rs`がDefaultを実装しない`Visible`も描画できることを検証する。成功出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach b91ecad && cargo check`でE0276を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0276の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0276](https://doc.rust-lang.org/error_codes/E0276.html)
