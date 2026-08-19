# E0308 調査記録：分岐とイテレータ具象型

## 目的

`statuses` がフラグに応じたステータス列を返す契約を、Rust 1.75.0で確認します。バグ状態では分岐が別の具体的イテレータ型を返すため、E0308で停止します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| Rust | `rustc 1.75.0` |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test`、`cargo run` |
| 再現境界 | `statuses` の`if`分岐で返すイテレータ具象型 |

## 最初に観測した事実

| 事実 | 証拠 |
| --- | --- |
| `if`側は配列由来のイテレータ、`else`側は`Once`です。 | バグ状態の`src/lib.rs` |
| `cargo check` は終了コード101でE0308を出します。 | `docs/observed-cargo-check-bug.txt` |
| 診断は分岐の型が互換でないことを示します。 | 同ファイル |

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| 返す要素の`&str`が異なる | 両分岐の要素型を確認する | 棄却。要素型はどちらも`&str`です。 |
| 分岐のイテレータ具象型が異なる | 両分岐を同じ`Box<dyn Iterator>`にする | 支持。E0308が消えました。 |

## 確定した原因

E0308は、期待される型と実際の型が一致しないときに発生します。[Rust error code E0308][1] `impl Iterator` の戻り値は呼び出しごとに異なる型を隠せますが、一つの関数本体では一つの具体型に決まる必要があります。配列イテレータと`Once`は別の型です。

## 最小修正

両分岐を同じトレイトオブジェクト型へそろえます。

```rust
pub fn statuses(include_closed: bool) -> Box<dyn Iterator<Item = &'static str>> {
    if include_closed {
        Box::new(["open", "closed"].into_iter())
    } else {
        Box::new(std::iter::once("open"))
    }
}
```

修正コミットは `f5f69ff` です。

## 回帰保証

`include_closed` が真と偽の両方を統合テストで確認し、2件が成功しました。`cargo run` は `["open", "closed"]` を出力します。

## 再現手順

```bash
git switch --detach 7ce4038
cargo check
git switch main
cargo test
```

## スコープと注意点

動的ディスパッチを使う最小修正です。静的ディスパッチが必要な性能要件や、イテレータ以外の分岐型統一には別の設計判断が必要です。

## References

[1]: https://doc.rust-lang.org/error_codes/E0308.html "Rust error code E0308"
