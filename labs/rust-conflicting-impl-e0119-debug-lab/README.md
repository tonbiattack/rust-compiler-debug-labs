# Rust E0119: 重複するトレイト実装

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

Taskは識別子を含む固有ラベルを返す。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`3f03659`では、`cargo check`が終了コード`101`でE0119を報告します。修正は`f9d121f`です。

```bash
git switch --detach 3f03659
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

同じ型に重なるトレイト実装だけを扱う。orphan ruleは扱わない。

## References

[1] [Rust Error Codes: E0119](https://doc.rust-lang.org/error_codes/E0119.html)
