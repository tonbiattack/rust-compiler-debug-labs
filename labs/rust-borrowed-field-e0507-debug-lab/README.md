# 共有参照のフィールド移動で発生する E0507 を読むデバッグラボ

`&Job` が指す`String`フィールドを所有値として返そうとして発生する E0507 を、必要なときだけ複製する最小デバッグ教材です。

## この題材で守る契約

> `duplicate_label(&job)` は独立したラベルを返し、呼び出し後も `job.label` を使えます。

## 最短の開始手順

```bash
cargo test
cargo run
```

## バグを再現する

```bash
cargo test
cargo run
```

修正後は統合テストが成功し、`cargo run` は `daily-report` を出力します。バグ状態のコミット `6d81aa3` で `cargo check` を実行すると、共有参照の背後にある`String`を移動できないE0507を再現できます。

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| フィールドの扱い | 所有権を移動しようとする | `clone`で独立した値を作る |
| 元の`Job` | コンパイル前に停止 | 呼び出し後も利用可能 |
| コンパイル | E0507で失敗 | `cargo check` が成功 |

## 前提条件

Rust 1.75.0、Cargo 1.75.0、およびRust標準ライブラリだけを使用します。

## スコープ

このラボは共有参照から`String`フィールドを所有値として返す条件を扱います。大きな値の複製コスト、`Cow`、可変借用からの置換は対象外です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0507.html "Rust error code E0507"
