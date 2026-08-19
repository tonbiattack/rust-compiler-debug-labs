# E0596 調査記録：共有参照を通じた更新

## 目的

ラベルに完了接尾辞を追加する契約をRust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test` |
| 再現境界 | `&String`に対する`push_str` |

## 最初に観測した事実

`cargo check`は終了コード101で、共有参照の背後の値を可変借用できないE0596を出しました。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| `push_str`が使えない | 引数だけを`&mut String`へ変更する | 支持。更新とテストが成功しました。 |

## 確定した原因

E0596は可変でない値を可変借用しようとしたときに発生します。[Rust error code E0596][1]

## 最小修正

```rust
pub fn mark_complete(label: &mut String) {
    label.push_str("-complete");
}
```

修正コミットは`f42a152`です。

## 回帰保証

接尾辞が追加される統合テストが成功しました。

## 再現手順

```bash
git switch --detach 5934b34
cargo check
git switch main
cargo test
```

## スコープと注意点

可変参照は呼び出し側に排他的な更新権限を要求します。読み取りだけのAPIには共有参照を維持します。

## References

[1]: https://doc.rust-lang.org/error_codes/E0596.html "Rust error code E0596"
