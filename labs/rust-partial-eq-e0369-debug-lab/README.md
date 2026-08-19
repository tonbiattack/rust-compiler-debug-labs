# 等値比較未実装で発生する E0369 を読むデバッグラボ

構造体へ`==`を適用して発生するE0369を、`PartialEq`導出で解く最小教材です。

## この題材で守る契約

> 二つのリリース時間枠が同じかを返します。

## 最短の開始手順

```bash
cargo test
```

## バグを再現する

バグ状態のコミット`bee10a9`で`cargo check`を実行するとE0369を再現できます。

## 観測の要約

| バグ状態 | 修正後 |
| --- | --- |
| `ReleaseWindow`へ`==`を適用できずE0369 | `PartialEq`を導出して比較可能 |

## References

[1]: https://doc.rust-lang.org/error_codes/E0369.html "Rust error code E0369"
