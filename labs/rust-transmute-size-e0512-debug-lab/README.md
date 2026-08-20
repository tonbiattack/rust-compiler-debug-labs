# Rust E0512: 異なるサイズのtransmute

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

u8のフラグを、値を保ったu16へ安全に変換する。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`5c7ba8d`では、`cargo check`が終了コード`101`でE0512を報告します。修正は`a24df7f`です。

```bash
git switch --detach 5c7ba8d
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

`transmute`のサイズ制約と数値変換だけを扱う。ビットパターンの再解釈は扱わない。

## References

[1] [Rust Error Codes: E0512](https://doc.rust-lang.org/error_codes/E0512.html)
