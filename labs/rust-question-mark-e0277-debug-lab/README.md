# `?`演算子の E0277 を読むデバッグラボ

`Result<u16, String>` を返す関数で `?` 演算子を使い、`ParseIntError` を返却用エラーへ変換できないために発生する E0277 を学ぶ、標準ライブラリだけの最小デバッグ教材です。

## この題材で守る契約

> `parse_port("8080")` は `Ok(8080)` を返し、不正な文字列にはポート番号を含むエラーを返します。

バグ状態では、`?` が必要とする `From<ParseIntError> for String` がないため、コンパイルが E0277 で停止します。

## 最短の開始手順

```bash
cargo test
cargo run
```

統合テスト2件が成功し、`cargo run` は `Ok(8080)` を出力します。

## バグを再現する

バグ状態はコミット `5ca2f6e` に保存しています。

```bash
git switch --detach 5ca2f6e
cargo check
```

`? couldn't convert the error to String` を含む E0277 が表示されます。確認後は修正済み状態へ戻ります。

```bash
git switch main
cargo test
```

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| エラー型 | `ParseIntError` を `String` へ変換できない | `map_err` で変換する |
| コンパイル | E0277で失敗 | `cargo check` が成功 |
| 正常入力 | 実行前に停止 | `Ok(8080)` |
| 不正入力 | 実行前に停止 | ポート番号を含むエラー |

詳しい観測と仮説比較は [docs/debugging-record.md](docs/debugging-record.md) に記録しています。

## 前提条件

| 項目 | 条件 |
| --- | --- |
| Rust | `rustc 1.75.0` |
| Cargo | `cargo 1.75.0` |
| 依存関係 | Rust標準ライブラリのみ |

## スコープ

このラボは `?` 演算子が `From` を使ってエラー型を変換する条件だけを扱います。独自エラー型の設計、複数エラーの集約、外部エラーライブラリは対象外です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0277.html "Rust error code E0277"
[2]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html "The Rust Programming Language: Recoverable Errors with Result"
