# Rust E0027: 構造体パターンのフィールド省略

Rust/Cargo 1.75系で検証した、構造体パターンの網羅性を扱う基礎ラボです。外部依存はありません。

## この題材で守る契約

`Task`から優先度だけを取り出す関数は`3`を返します。`title`を使わない意図は`..`で明示します。

## 最短の開始手順

```bash
cargo check
cargo test
```

修正済みの既定ブランチでは、上記が成功します。

## バグを再現する

バグ状態コミット`815a513`へ切り替えると、`cargo check`は終了コード`101`でE0027を出します。

```bash
git switch --detach 815a513
cargo check
```

修正はコミット`5504d39`です。元の作業ブランチへ戻るには`git switch main`を実行します。

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 最小実装 |
| `tests/priority.rs` | 優先度の契約テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測から回帰確認までの記録 |

## スコープ

このラボは構造体パターンで不要フィールドを無視する規則だけを扱います。構造体の所有権移動や借用規則は扱いません。

## References

[1] [Rust Error Codes: E0027](https://doc.rust-lang.org/error_codes/E0027.html)
