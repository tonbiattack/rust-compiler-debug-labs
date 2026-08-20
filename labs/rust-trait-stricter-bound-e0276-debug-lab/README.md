# Rust E0276: トレイト実装の過剰な型境界

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

`Render`はDisplayを実装する任意の値を角括弧付き文字列へ変換する。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`b91ecad`では、`cargo check`が終了コード`101`でE0276を報告します。修正は`2a735d0`です。

```bash
git switch --detach b91ecad
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

トレイト実装におけるメソッド境界の互換性だけを扱う。

## References

[1] [Rust Error Codes: E0276](https://doc.rust-lang.org/error_codes/E0276.html)
