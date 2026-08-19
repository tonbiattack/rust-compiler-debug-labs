# 型推論不足で発生する E0282 を読むデバッグラボ

要素型が不明な空の`Vec`を生成して発生するE0282を、型注釈で解く最小教材です。

## この題材で守る契約

> 新規の処理待ちキューが空であることを返します。

## 最短の開始手順

```bash
cargo test
```

## バグを再現する

バグ状態のコミット`7d2aa25`で`cargo check`を実行するとE0282を再現できます。

## 観測の要約

| バグ状態 | 修正後 |
| --- | --- |
| `Vec<T>`の`T`を推論できずE0282 | `Vec<String>`を明示して成功 |

## References

[1]: https://doc.rust-lang.org/error_codes/E0282.html "Rust error code E0282"
