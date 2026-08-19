# E0624 調査記録

## 目的

タスク題名を表示用に取得する。 をRust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
|---|---|
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test` |
| 再現境界 | 私有メソッド呼び出しで発生する E0624 |

## 最初に観測した事実

`cargo check`は終了コード101でE0624を出しました。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
|---|---|---|
| 契約と実装の不一致 | 直接原因だけを変更 | 修正後にテストが成功しました。 |

## 確定した原因

E0624の公式規則に反する実装でした。[Rust error code E0624][1]

## 最小修正

意図した公開APIとしてメソッドを公開する。

## 回帰保証

統合テストが成功しました。

## 再現手順

```bash
git switch --detach d0b41c0
cargo check
git switch main
cargo test
```

## スコープと注意点

このラボは診断の直接原因だけを扱います。

## References

[1]: https://doc.rust-lang.org/error_codes/E0624.html "Rust error code E0624"
