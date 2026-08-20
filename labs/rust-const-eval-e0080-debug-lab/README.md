# Rust E0080: 定数評価の整数オーバーフロー

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

再試行待機時間300ミリ秒を、コンパイル時に評価可能な型で表現する。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`3420faa`では、`cargo check`が終了コード`101`でE0080を報告します。修正は`0b52ed0`です。

```bash
git switch --detach 3420faa
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

定数式の評価可能性だけを扱う。実行時の算術エラーは扱わない。

## References

[1] [Rust Error Codes: E0080](https://doc.rust-lang.org/error_codes/E0080.html)
