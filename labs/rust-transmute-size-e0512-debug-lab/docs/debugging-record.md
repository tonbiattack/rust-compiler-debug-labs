# E0512 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、u8のフラグをu16にする公開関数を最小境界にした。

## 最初に観測した事実

バグコミット`5c7ba8d`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0512]: cannot transmute between types of different sizes`を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| unsafeブロックがない | unsafeを付ける | 既にunsafe | 直接原因ではない | 棄却 |
| 型サイズが異なる | 数値変換へ変更 | cargo check成功 | 診断消失 | 採用 |

## 確定した原因

`transmute`は異なるサイズの型の間では使えない。[1]

## 最小修正

`transmute`を削除し、`u16::from(flag)`で数値変換した。

## 回帰保証

`tests/level.rs`が7をu16の7へ変換することを検証する。成功出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 5c7ba8d && cargo check`でE0512を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0512の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0512](https://doc.rust-lang.org/error_codes/E0512.html)
