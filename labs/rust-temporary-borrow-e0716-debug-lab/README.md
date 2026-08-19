# 一時値の借用で発生する E0716 を読むデバッグラボ

関数呼び出しで生成した一時的な `String` を借用し、その参照を次の文で使うと発生する E0716 を学ぶ最小デバッグ教材です。

## この題材で守る契約

> `selected_length` は生成した `"fast,safe"` の最初の区切り要素の文字数として `4` を返します。

バグ状態では `&make_line()` が作る一時値が文末で破棄されるため、参照を使う前にコンパイルが E0716 で停止します。

## 最短の開始手順

```bash
cargo test
cargo run
```

統合テストが成功し、`cargo run` は `4` を出力します。

## バグを再現する

バグ状態はコミット `4dfe819` に保存しています。

```bash
git switch --detach 4dfe819
cargo check
```

`temporary value dropped while borrowed` を含む E0716 が表示されます。確認後は `git switch main` で修正済み状態へ戻ります。

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| 被借用値 | `make_line()` が作る一時値 | `line` というローカル変数 |
| コンパイル | E0716で失敗 | `cargo check` が成功 |
| 関数の戻り値 | 実行前に停止 | `4` |

## 前提条件

| 項目 | 条件 |
| --- | --- |
| Rust | `rustc 1.75.0` |
| Cargo | `cargo 1.75.0` |
| 依存関係 | Rust標準ライブラリのみ |

## スコープ

このラボは、文中の一時値を参照したまま次の文で使う条件だけを扱います。参照を返すAPI、ライフタイム注釈、スコープ付きスレッドは対象外です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0716.html "Rust error code E0716"
