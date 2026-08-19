# E0716 調査記録：一時値の借用

## 目的

生成した `"fast,safe"` の最初の要素の文字数として `4` を返す契約をRust 1.75.0で確認します。バグ状態では、関数呼び出しが作る一時的な `String` を参照したまま次の文で使うため、E0716が発生します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 公開境界 | `selected_length() -> usize` |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo check`、`cargo test`、`cargo run` |
| 依存関係 | Rust標準ライブラリのみ |
| 最終観測 | 返り値が最初の区切り要素の文字数 `4` であること |

## 最初に観測した事実

| 事実 | 証拠 |
| --- | --- |
| `&make_line()` の結果を `selected` へ保存しました。 | バグ状態の `src/lib.rs` |
| 次の文で `selected.len()` を呼びました。 | 同ファイル |
| `cargo check` は終了コード101でE0716を出しました。 | `docs/observed-cargo-check-bug.txt` |
| 診断はより長く生きる `let` 束縛を提案しました。 | 同ファイルの `help` |

バグ状態のコミットは `4dfe819` です。

## 競合仮説と検証

| 仮説 | 最小検証 | 結果 |
| --- | --- | --- |
| `split` が参照を返すため使えない | 明示的な `String` 変数を借用して同じ `split` を呼ぶ | 棄却。`split` 自体は借用中の文字列から参照を返せます。 |
| 一時値の破棄が早すぎる | `make_line()` の結果を `let line` へ束縛してから借用する | 支持。E0716が消え、テストが通りました。 |
| 返り値の `usize` が原因 | 借用を使う前の型を変えず、束縛だけを追加する | 棄却。戻り型を変えずに修正できました。 |

## 確定した原因

E0716は、一時値が借用中にもかかわらず破棄される場合に発生します。[Rust error code E0716][1] 通常、一時値は生成された文の終わりに破棄されます。`first_segment(&make_line())` の直後に一時的な `String` は破棄されますが、`selected` は次の文まで使われるため、参照が無効になり得ます。

## 最小修正

修正は、所有する文字列をローカル変数へ束縛してから借用することです。

```rust
let line = make_line();
let selected = first_segment(&line);
```

`line` は関数ブロックの終わりまで生きるため、`selected.len()` まで参照が有効です。修正コミットは `0a8c481` です。

## 回帰保証

| 守ること | テスト | 結果 |
| --- | --- | --- |
| 最初の区切り要素の文字数を返す | `returns_the_length_of_the_first_segment` | 成功 |

`cargo test` は統合テストを成功させ、`cargo run` は `4` を出力しました。

## 再現手順

```bash
cargo test

git switch --detach 4dfe819
cargo check

git switch main
```

## スコープと注意点

このラボは関数呼び出しで作った一時的な `String` を、次の文で借用する条件だけを確認しています。参照を返す公開API、複雑な一時値寿命延長規則、別のRustバージョンの詳細には同じ結論を自動的に広げません。

## References

[1]: https://doc.rust-lang.org/error_codes/E0716.html "Rust error code E0716"
