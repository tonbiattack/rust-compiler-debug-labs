# E0507 調査記録：共有参照越しのフィールド移動

## 目的

共有参照の`Job`から所有するラベルを取得しつつ、元の`Job`を使い続ける契約をRust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| Rust | `rustc 1.75.0` |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test`、`cargo run` |
| 再現境界 | `&Job`越しに`String`フィールドを所有値として返す箇所 |

## 最初に観測した事実

| 事実 | 証拠 |
| --- | --- |
| `duplicate_label` は `&Job` を受け取ります。 | バグ状態の`src/lib.rs` |
| `label` は`String`であり`Copy`ではありません。 | `Job`の定義 |
| `cargo check` は終了コード101でE0507を出します。 | `docs/observed-cargo-check-bug.txt` |

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| `String`を共有参照から直接返せる | `job.label` をそのまま返す | 棄却。所有権移動が必要になりE0507です。 |
| 複製すれば元の`Job`を保持できる | `job.label.clone()` へ置換する | 支持。コンパイルとテストが成功しました。 |

## 確定した原因

E0507は借用された値から移動したときに発生します。[Rust error code E0507][1] `&Job` はフィールドを読む権限を与えますが、`String`の所有権を持ち出す権限は与えません。

## 最小修正

返却値を所有値にする契約を保ち、フィールドを複製します。

```rust
pub fn duplicate_label(job: &Job) -> String {
    job.label.clone()
}
```

修正コミットは `8a57087` です。

## 回帰保証

統合テストで返却値が期待どおりであることと、呼び出し後も`job.label`が利用可能であることを確認しました。`cargo run` は `daily-report` を出力します。

## 再現手順

```bash
git switch --detach 6d81aa3
cargo check
git switch main
cargo test
```

## スコープと注意点

`clone`は小さなラベル文字列には明快ですが、大きなデータを高頻度で複製する設計を推奨するものではありません。所有権を渡せるなら引数を所有値にする、借用で十分なら`&str`を返す方法も検討します。

## References

[1]: https://doc.rust-lang.org/error_codes/E0507.html "Rust error code E0507"
