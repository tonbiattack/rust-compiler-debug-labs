# Rust E0184: Drop型へのCopy実装

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

チケットを2人の利用者へ渡すとき、Dropを持つ値は`clone`で明示的に複製する。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`704d604`では、`cargo check`が終了コード`101`でE0184を報告します。修正は`4f3f669`です。

```bash
git switch --detach 704d604
cargo check
git switch main
```

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 修正済み最小実装 |
| `tests/` | 公開APIの統合テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 調査記録 |

## スコープ

DropとCopyの排他性だけを扱う。Drop順序や資源解放の実装詳細は扱わない。

## References

[1] [Rust Error Codes: E0184](https://doc.rust-lang.org/error_codes/E0184.html)
