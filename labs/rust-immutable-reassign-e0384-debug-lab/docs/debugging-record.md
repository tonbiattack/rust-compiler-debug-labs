# E0384 調査記録

## 目的

待機時間を10秒へ上限設定する。 をRust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
|---|---|
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test` |
| 再現境界 | 不変な再代入で発生する E0384 |

## 最初に観測した事実

`cargo check`は終了コード101でE0384を出しました。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
|---|---|---|
| 契約と実装の不一致 | 直接原因だけを変更 | 修正後にテストが成功しました。 |

## 確定した原因

E0384の公式規則に反する実装でした。[Rust error code E0384][1]

## 最小修正

`mut`で再代入可能にする。

## 回帰保証

統合テストが成功しました。

## 再現手順

```bash
git switch --detach b34d30d
cargo check
git switch main
cargo test
```

## スコープと注意点

このラボは診断の直接原因だけを扱います。

## References

[1]: https://doc.rust-lang.org/error_codes/E0384.html "Rust error code E0384"
