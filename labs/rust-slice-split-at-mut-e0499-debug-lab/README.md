# `split_at_mut` の E0499 を読むデバッグラボ

同一スライスの添字アクセスから二つの可変参照を取ろうとして発生する E0499 を、`split_at_mut` による非重複な部分スライスで解決する最小デバッグ教材です。

## この題材で守る契約

> `overwrite_first_two` は先頭2要素を `10` と `20` に更新し、それ以外の要素を保持します。

バグ状態では、`values[0]` と `values[1]` から同時に可変参照を作るため、コンパイラが E0499 で停止します。

## 最短の開始手順

```bash
cargo test
cargo run
```

統合テスト2件が成功し、`cargo run` は `[10, 20, 0]` を出力します。

## バグを再現する

バグ状態はコミット `61ce9c8` に保存しています。

```bash
git switch --detach 61ce9c8
cargo check
```

`cannot borrow values[_] as mutable more than once at a time` を含む E0499 が表示されます。確認後は `git switch main` で修正済み状態へ戻ります。

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| 可変参照 | 添字アクセスから二つ作ろうとする | 非重複な部分スライスから一つずつ取る |
| コンパイル | E0499で失敗 | `cargo check` が成功 |
| 3要素のスライス | 実行前に停止 | `[10, 20, 3]` |
| 2要素のスライス | 実行前に停止 | `[10, 20]` |

## 前提条件

| 項目 | 条件 |
| --- | --- |
| Rust | `rustc 1.75.0` |
| Cargo | `cargo 1.75.0` |
| 依存関係 | Rust標準ライブラリのみ |

## スコープ

このラボは、同一スライスで二つの要素を同時に可変更新する場合だけを扱います。共有可変状態、並行処理、`unsafe`、可変参照一般のすべてを扱うものではありません。

## References

[1]: https://doc.rust-lang.org/error_codes/E0499.html "Rust error code E0499"
[2]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut "slice::split_at_mut"
[3]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html "The Rust Programming Language: References and Borrowing"
