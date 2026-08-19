# E0277 調査記録：`?`演算子のエラー変換

## 目的

`parse_port("8080")` が `Ok(8080)` を返し、不正な入力をエラーとして返す契約を、Rust 1.75.0 で確認します。バグ状態では `?` 演算子のエラー変換が不足し、コンパイル前に E0277 が発生します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 公開境界 | `parse_port(input: &str) -> Result<u16, String>` |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo check`、`cargo test`、`cargo run` |
| 依存関係 | Rust標準ライブラリのみ |
| 最終観測 | 有効なポート番号と不正入力のエラーメッセージ |

## 最初に観測した事実

| 事実 | 証拠 |
| --- | --- |
| `input.parse()` のエラー型は `ParseIntError` です。 | E0277診断の `FromResidual` と `ParseIntError` の記述 |
| 関数は `Result<u16, String>` を返します。 | `src/lib.rs` |
| `cargo check` は終了コード101でE0277を出しました。 | `docs/observed-cargo-check-bug.txt` |
| 診断は `?` が `From` を使うエラー変換を必要とすると示しました。 | 同ファイルの `note` と `help` |

バグ状態のコミットは `5ca2f6e` です。

## 競合仮説と検証

| 仮説 | 最小検証 | 結果 |
| --- | --- | --- |
| `?` を関数内で使えない | 関数はすでに `Result` を返していることを確認する | 棄却。戻り型そのものは `?` と互換です。 |
| `u16` の解析ができない | `"8080".parse::<u16>()` の成功を確認する | 棄却。成功値の型は問題ではありません。 |
| `ParseIntError` から `String` への変換がない | `map_err` で変換して同じテストを実行する | 支持。E0277が消え、テストが通りました。 |

## 確定した原因

`?` は `Err` を返すとき、現在の関数の戻りエラー型へ `From` を通じて変換します。[The Rust Programming Language: Recoverable Errors with Result][2] この関数では `ParseIntError` を `String` に変換する実装がないため、E0277になりました。[Rust error code E0277][1]

## 最小修正

修正は、解析エラーを `map_err` で返却用の `String` に変換することです。

```rust
let port: u16 = input
    .parse()
    .map_err(|error| format!("ポート番号を解析できません: {error}"))?;
```

修正コミットは `a2a66a7` です。独自エラー型や外部クレートは追加していません。

## 回帰保証

| 守ること | テスト | 結果 |
| --- | --- | --- |
| 正常なポート番号を解析する | `parses_a_valid_port_number` | 成功 |
| 不正入力を読みやすいエラーにする | `returns_a_readable_error_for_invalid_input` | 成功 |

`cargo test` は統合テスト2件を成功させ、`cargo run` は `Ok(8080)` を出力しました。

## 再現手順

```bash
cargo test

git switch --detach 5ca2f6e
cargo check

git switch main
```

## スコープと注意点

このラボは `ParseIntError` を `String` へ変換する一つの関数だけを確認しています。複数エラー型を維持する設計、エラー情報の国際化、外部エラーライブラリの選定には、そのまま一般化しません。

## References

[1]: https://doc.rust-lang.org/error_codes/E0277.html "Rust error code E0277"
[2]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html "The Rust Programming Language: Recoverable Errors with Result"
