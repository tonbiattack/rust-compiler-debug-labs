# トレイト未導入で発生する E0599 を読むデバッグラボ

`Cursor`が実装する`Write`トレイトをスコープへ導入せず、`write_all`を呼んで発生する E0599 を学ぶ最小教材です。

## この題材で守る契約

> メモリバッファへ `ready` を書き込み、文字列として返します。

## 最短の開始手順

```bash
cargo test
```

修正後は統合テストが成功します。

## バグを再現する

バグ状態のコミット `bac7583` で `cargo check` を実行します。

```bash
git switch --detach bac7583
cargo check
git switch main
```

`no method named write_all found` を含む E0599 が表示されます。

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| メソッド解決 | `Write`がスコープ外 | `use std::io::Write` |
| コンパイル | E0599で失敗 | `cargo check` が成功 |
| 出力 | 実行前に停止 | `ready` |

## スコープ

このラボは標準ライブラリのトレイトメソッドを使う条件だけを扱います。独自トレイトの実装、同名メソッドの優先順位、非同期I/Oは対象外です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0599.html "Rust error code E0599"
