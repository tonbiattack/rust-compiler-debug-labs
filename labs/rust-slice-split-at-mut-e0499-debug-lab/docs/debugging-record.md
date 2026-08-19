# E0499 調査記録：添字アクセスと `split_at_mut`

## 目的

スライスの先頭2要素を更新し、残りを保持する契約をRust 1.75.0で確認します。バグ状態では同じスライスから二つの可変参照を取得し、E0499が発生します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 公開境界 | `overwrite_first_two(values: &mut [i32])` |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo check`、`cargo test`、`cargo run` |
| 依存関係 | Rust標準ライブラリのみ |
| 最終観測 | スライスの先頭2要素と、以降の要素が保持されること |

## 最初に観測した事実

| 事実 | 証拠 |
| --- | --- |
| `&mut values[0]` の取得後、`&mut values[1]` でE0499になりました。 | `docs/observed-cargo-check-bug.txt` |
| 診断は最初の可変借用と二つ目の可変借用を示しました。 | 同ファイルの関連行 |
| `cargo check` と `cargo test` は終了コード101でした。 | 保存済み観測出力 |

バグ状態のコミットは `61ce9c8` です。

## 競合仮説と検証

| 仮説 | 最小検証 | 結果 |
| --- | --- | --- |
| 2要素が存在しないため失敗する | 要素数が3の固定入力で診断を確認する | 棄却。要素数は十分でもコンパイル時に失敗しました。 |
| 借用の寿命を短くすれば解決する | 先頭の参照を使い終えてから二つ目を作る | 一部支持。ただし同時に二つを更新する契約を満たしません。 |
| 二つの領域が非重複だとAPIで示せば解決する | `split_at_mut(1)` に置換する | 支持。コンパイルと統合テストが成功しました。 |

## 確定した原因

Rustでは、同じデータに同時に複数の可変参照を持てません。[The Rust Programming Language: References and Borrowing][3] 添字式だけでは、コンパイラは二つの可変参照が非重複であることを一般に保証できないため、E0499になりました。[Rust error code E0499][1]

## 最小修正

修正は、`split_at_mut(1)` でスライスを先頭1要素と残りへ分割することです。

```rust
let (first_part, remaining) = values.split_at_mut(1);
first_part[0] = 10;
remaining[0] = 20;
```

`split_at_mut` は分割位置を境に別々の可変部分スライスを返します。[slice::split_at_mut][2] 修正コミットは `1bafde0` です。

## 回帰保証

| 守ること | テスト | 結果 |
| --- | --- | --- |
| 先頭2要素だけを更新する | `overwrites_the_first_two_values_without_changing_the_rest` | 成功 |
| 要素数がちょうど2でも更新する | `accepts_a_slice_with_exactly_two_values` | 成功 |

`cargo test` は統合テスト2件を成功させ、`cargo run` は `[10, 20, 0]` を出力しました。

## 再現手順

```bash
cargo test

git switch --detach 61ce9c8
cargo check

git switch main
```

## スコープと注意点

このラボは固定した分割位置で連続する二領域を更新する条件だけを確認しています。任意個数の添字、重複の検出、並行更新、境界外アクセスの設計には別の検討が必要です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0499.html "Rust error code E0499"
[2]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut "slice::split_at_mut"
[3]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html "The Rust Programming Language: References and Borrowing"
