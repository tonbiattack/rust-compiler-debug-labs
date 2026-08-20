# Rust E0133: unsafe関数の未保護呼び出し

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

公開関数は、安全性根拠を局所化した最小のunsafe境界で固定マーカーを返す。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`f940878`では、`cargo check`が終了コード`101`でE0133を報告します。修正は`184a9ea`です。

```bash
git switch --detach f940878
cargo check
git switch main
```

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 修正済み最小実装 |
| `tests/` | 公開APIの統合テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測、原因、修正、回帰保証 |

## スコープ

unsafeブロックの必要性だけを扱う。ポインタ操作やFFIは扱わない。

## References

[1] [Rust Error Codes: E0133](https://doc.rust-lang.org/error_codes/E0133.html)
