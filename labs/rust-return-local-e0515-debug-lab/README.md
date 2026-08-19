# ローカル参照の返却で発生する E0515 を読むデバッグラボ

関数内で生成した`String`への`&str`を返そうとして発生する E0515 を、所有する`String`を返す設計へ直す最小デバッグ教材です。

## この題材で守る契約

> `normalize_label` は前後の空白を除き、小文字化したラベルを返します。

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

修正後は統合テスト2件が成功し、`cargo run` は `daily-report` を出力します。バグ状態のコミット `4696e78` で `cargo check` を実行すると、ローカル変数への参照を返せないE0515を再現できます。

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| 返り値 | ローカル`String`への`&str` | 所有する`String` |
| 借用元 | 関数終了時に破棄される | 値とともに呼び出し元へ移る |
| コンパイル | E0515で失敗 | `cargo check` が成功 |

## 前提条件

Rust 1.75.0、Cargo 1.75.0、およびRust標準ライブラリだけを使用します。

## スコープ

このラボは関数ローカルの文字列を正規化して返す条件を扱います。参照を返すAPIでのライフタイム設計、文字列のUnicode正規化は対象外です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0515.html "Rust error code E0515"
