# E0599 調査記録：トレイトメソッドとスコープ

## 目的

メモリ上の`Cursor`へ状態文字列を書き込む契約を、Rust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test` |
| 再現境界 | `Cursor`に対する`write_all`呼び出し |

## 最初に観測した事実

`cargo check`は終了コード101でE0599を出し、`write_all`を提供するトレイトがスコープにないことを`help`で示しました。記録は`docs/observed-cargo-check-bug.txt`です。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| `Cursor`が書き込みを実装しない | `Write`を導入して同じコードを実行する | 棄却。実装自体は存在しました。 |
| トレイトがスコープにない | `use std::io::{Cursor, Write};`へ置換する | 支持。E0599が消えました。 |

## 確定した原因

E0599は型にメソッドが見つからないときに発生します。[Rust error code E0599][1] 今回の`write_all`は`Cursor`の固有メソッドではなく`Write`トレイトのメソッドであり、呼び出すモジュールへトレイトを導入する必要があります。

## 最小修正

```rust
use std::io::{Cursor, Write};
```

修正コミットは `28eaeff` です。

## 回帰保証

`format_status()`が`ready`を返す統合テストが成功しました。

## 再現手順

```bash
git switch --detach bac7583
cargo check
git switch main
cargo test
```

## スコープと注意点

E0599はトレイト未導入以外にも、型がメソッドを実装しない場合に発生します。診断の`help`に候補トレイトがあるかを確認してから実装追加を判断します。

## References

[1]: https://doc.rust-lang.org/error_codes/E0599.html "Rust error code E0599"
