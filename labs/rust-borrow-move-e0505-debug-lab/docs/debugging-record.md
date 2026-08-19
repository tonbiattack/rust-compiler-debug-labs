# E0505 調査記録：借用中の値の移動

## 目的

ラベルの先頭語と完全なラベルを連結する契約を、Rust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test` |
| 再現境界 | `label`の部分借用後の所有権移動 |

## 最初に観測した事実

`cargo check`は終了コード101でE0505を出し、`prefix`が後で使われる間は`label`を移動できないと示しました。記録は`docs/observed-cargo-check-bug.txt`です。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| 部分文字列の借用が問題 | 借用を保ち、`label`の移動だけをやめる | 支持。E0505が消えました。 |
| `String`を必ず所有値にする必要がある | `format!`へ`label`を参照として渡す | 棄却。最終出力は新しい`String`であり、`label`自体の移動は不要です。 |

## 確定した原因

E0505は値が借用中のまま移動されたときに発生します。[Rust error code E0505][1] `prefix`は`label`を参照し、後続の`format!`で使われるため、先に`label`を`owned_label`へ移動できません。

## 最小修正

```rust
let prefix = &label[..5];
format!("{prefix}: {label}")
```

`format!`は二つの値を共有借用して結果の`String`を作ります。修正コミットは `6dfb67e` です。

## 回帰保証

`decorate_label()`が`daily: daily-report`を返す統合テストが成功しました。

## 再現手順

```bash
git switch --detach dbeccab
cargo check
git switch main
cargo test
```

## スコープと注意点

移動が本当に必要なら、借用を最後に使ってからスコープを終える、または値を複製するなどの設計が必要です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0505.html "Rust error code E0505"
