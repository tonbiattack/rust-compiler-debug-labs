# Rust E0191: 関連型未指定のトレイトオブジェクト

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

数値ストレージをトレイトオブジェクト越しに読み取るとき、関連型`Item`を`u32`として指定する。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`b82342d`では、`cargo check`が終了コード`101`でE0191を報告します。修正はコミット`a3f6e99`です。

```bash
git switch --detach b82342d
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

関連型を持つ`dyn`トレイトの型指定だけを扱う。dyn互換性そのものは扱わない。

## References

[1] [Rust Error Codes: E0191](https://doc.rust-lang.org/error_codes/E0191.html)
