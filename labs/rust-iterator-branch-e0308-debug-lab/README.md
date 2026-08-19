# 分岐したイテレータで発生する E0308 を読むデバッグラボ

`if` の各分岐で異なる具体的なイテレータを返そうとして発生する E0308 を、トレイトオブジェクトへそろえる最小デバッグ教材です。

## この題材で守る契約

> `statuses(true)` は `open` と `closed` を、`statuses(false)` は `open` だけを返します。

## 最短の開始手順

```bash
cargo test
cargo run
```

## バグを再現する

```bash
cargo test
cargo run
```

修正後は統合テスト2件が成功し、`cargo run` は `["open", "closed"]` を出力します。バグ状態のコミット `7ce4038` へ切り替えて `cargo check` を実行すると、`if` と `else` の型が互換でないE0308を再現できます。

```bash
git switch --detach 7ce4038
cargo check
git switch main
```

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| 分岐の型 | 配列イテレータと`Once` | `Box<dyn Iterator<...>>` |
| コンパイル | E0308で失敗 | `cargo check` が成功 |
| 出力 | 実行前に停止 | 要求に応じたステータス列 |

## 前提条件

Rust 1.75.0、Cargo 1.75.0、およびRust標準ライブラリだけを使用します。

## スコープ

このラボは、異なる具体的イテレータを一つの分岐式から返す場合を扱います。列挙の性能比較、外部の`Either`型、非同期ストリームは対象外です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0308.html "Rust error code E0308"
