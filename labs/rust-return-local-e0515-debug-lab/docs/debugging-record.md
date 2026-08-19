# E0515 調査記録：ローカル値への参照を返すAPI

## 目的

入力文字列をトリムして小文字化した結果を安全に返す契約を、Rust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| Rust | `rustc 1.75.0` |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test`、`cargo run` |
| 再現境界 | 関数ローカルの`String`への参照を返す箇所 |

## 最初に観測した事実

| 事実 | 証拠 |
| --- | --- |
| `to_lowercase` は新しい`String`を作ります。 | バグ状態の`src/lib.rs` |
| `normalized` は関数ローカル変数です。 | 同ファイル |
| `cargo check` は終了コード101でE0515を出します。 | `docs/observed-cargo-check-bug.txt` |

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| `as_str`の呼び出し方が問題 | `as_str`を使ったまま返り値だけを所有値にする | 棄却。問題はメソッドではなく借用元の寿命です。 |
| ローカル値を所有値として返せばよい | `String`をそのまま返す | 支持。E0515が消えました。 |

## 確定した原因

E0515はローカル変数への参照を返したときに発生します。[Rust error code E0515][1] ローカル変数は関数終了時に破棄されるため、その参照を呼び出し元へ返すと無効な参照になり得ます。

## 最小修正

正規化結果を所有する`String`として返します。

```rust
pub fn normalize_label(input: &str) -> String {
    input.trim().to_lowercase()
}
```

修正コミットは `40be402` です。

## 回帰保証

トリムと小文字化、内部空白の保持を統合テスト2件で確認しました。`cargo run` は `daily-report` を出力します。

## 再現手順

```bash
git switch --detach 4696e78
cargo check
git switch main
cargo test
```

## スコープと注意点

所有値を返すことで安全性を得ますが、呼び出しごとに新しい文字列を生成します。借用した入力の部分文字列だけで済む問題なら、元の入力を参照として返す別設計が適切です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0515.html "Rust error code E0515"
