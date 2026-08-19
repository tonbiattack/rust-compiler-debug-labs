# E0369 調査記録：構造体の等値比較

## 目的

二つのリリース時間枠が同じかを判定する契約をRust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test` |
| 再現境界 | `ReleaseWindow`への`==`適用 |

## 最初に観測した事実

`cargo check`は終了コード101で、`ReleaseWindow`に`==`を適用できないE0369を出しました。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| フィールド値が比較不能 | 構造体だけに`PartialEq`を導出する | 支持。比較とテストが成功しました。 |

## 確定した原因

E0369は対象型が二項演算をサポートしないときに発生します。[Rust error code E0369][1]

## 最小修正

```rust
#[derive(PartialEq)]
pub struct ReleaseWindow { /* ... */ }
```

修正コミットは`521a7f2`です。

## 回帰保証

同じ時間枠と異なる時間枠の統合テスト2件が成功しました。

## 再現手順

```bash
git switch --detach bee10a9
cargo check
git switch main
cargo test
```

## スコープと注意点

`PartialEq`は等値比較だけを導入します。並び順が必要なら`PartialOrd`または`Ord`を別途検討します。

## References

[1]: https://doc.rust-lang.org/error_codes/E0369.html "Rust error code E0369"
