# 借用中の移動で発生する E0505 を読むデバッグラボ

文字列の一部を借用したまま文字列自体を移動して発生する E0505 を、移動を避けて解決する最小教材です。

## この題材で守る契約

> `decorate_label()` は `daily: daily-report` を返します。

## 最短の開始手順

```bash
cargo test
```

修正後は統合テストが成功します。

## バグを再現する

バグ状態のコミット `dbeccab` で `cargo check` を実行します。

```bash
git switch --detach dbeccab
cargo check
git switch main
```

`cannot move out of label because it is borrowed` を含む E0505 が表示されます。

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| `label` | 部分借用後に移動 | 書式化時も共有借用 |
| コンパイル | E0505で失敗 | `cargo check` が成功 |
| 契約 | 実行前に停止 | 装飾済みラベルを返す |

## スコープ

このラボは同一スコープでの局所的な借用と移動だけを扱います。`Rc`、並行処理、可変借用との競合は対象外です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0505.html "Rust error code E0505"
