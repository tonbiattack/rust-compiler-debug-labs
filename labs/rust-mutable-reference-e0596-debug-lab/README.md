# 可変参照不足で発生する E0596 を読むデバッグラボ

共有参照を通じて文字列を更新して発生するE0596を、可変参照を明示して解く最小教材です。

## この題材で守る契約

> ラベルへ`-complete`を追加します。

## 最短の開始手順

```bash
cargo test
```

## バグを再現する

バグ状態のコミット`5934b34`で`cargo check`を実行するとE0596を再現できます。

## 観測の要約

| バグ状態 | 修正後 |
| --- | --- |
| `&String`から更新しようとしてE0596 | `&mut String`を受け取り更新 |

## References

[1]: https://doc.rust-lang.org/error_codes/E0596.html "Rust error code E0596"
