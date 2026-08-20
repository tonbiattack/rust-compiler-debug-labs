# Rust E0207: 制約されないimpl型パラメータ

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

呼び出し側が選ぶ型の既定値を返すとき、型パラメータはメソッド側で宣言する。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`4e1376a`では、`cargo check`が終了コード`101`でE0207を報告します。修正はコミット`56d0736`です。

```bash
git switch --detach 4e1376a
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

`impl`の型パラメータ制約だけを扱う。ジェネリックAPI全般の設計は扱わない。

## References

[1] [Rust Error Codes: E0207](https://doc.rust-lang.org/error_codes/E0207.html)
