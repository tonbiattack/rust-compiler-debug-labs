# Rust E0063: 構造体初期化時のフィールド省略

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

日次タスクを生成するとき、題名と通常優先度をすべて指定する。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`2633227`では、`cargo check`が終了コード`101`でE0063を報告します。修正はコミット`c69a940`です。

```bash
git switch --detach 2633227
cargo check
git switch main
```

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 修正済みの最小実装 |
| `tests/` | 公開APIの統合テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測、原因、最小修正、回帰保証 |

## スコープ

構造体リテラルの完全性だけを扱う。構造体パターンの分解規則は扱わない。

## References

[1] [Rust Error Codes: E0063](https://doc.rust-lang.org/error_codes/E0063.html)
