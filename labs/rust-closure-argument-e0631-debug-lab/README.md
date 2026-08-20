# Rust E0631: クロージャ引数の型不一致

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

カウント3へ変換処理を適用して4を返す。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`4ba2c36`では、`cargo check`が終了コード`101`でE0631を報告します。修正は`ba94c5a`です。

```bash
git switch --detach 4ba2c36
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

高階関数とクロージャ引数の型一致だけを扱う。

## References

[1] [Rust Error Codes: E0631](https://doc.rust-lang.org/error_codes/E0631.html)
