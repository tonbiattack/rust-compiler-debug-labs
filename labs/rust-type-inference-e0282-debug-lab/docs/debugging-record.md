# E0282 調査記録：空のベクターの要素型

## 目的

空の処理待ちキューを作る契約をRust 1.75.0で確認します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 失敗コマンド | `cargo check` |
| 成功コマンド | `cargo test` |
| 再現境界 | `Vec::new()`の要素型 |

## 最初に観測した事実

`cargo check`は終了コード101で、`Vec<T>`に型注釈が必要だというE0282を出しました。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| `is_empty`の利用が不足 | 要素型だけを注釈する | 支持。コンパイルが成功しました。 |

## 確定した原因

E0282は型推論が一意の型を決められないときに発生します。[Rust error code E0282][1]

## 最小修正

```rust
let queue: Vec<String> = Vec::new();
```

修正コミットは`7536fde`です。

## 回帰保証

新規キューが空である統合テストが成功しました。

## 再現手順

```bash
git switch --detach 7d2aa25
cargo check
git switch main
cargo test
```

## スコープと注意点

型注釈は必要な曖昧さだけを解消します。実データがあれば、その利用から型を推論できることもあります。

## References

[1]: https://doc.rust-lang.org/error_codes/E0282.html "Rust error code E0282"
