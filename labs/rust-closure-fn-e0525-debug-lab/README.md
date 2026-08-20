# Rust E0525: FnOnceクロージャをFnとして渡す失敗

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

同じラベルを生成するクロージャを2回呼び、二つの同じ文字列を返す。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`09b82a1`では、`cargo check`が終了コード`101`でE0525を報告します。修正は`67b7a63`です。

```bash
git switch --detach 09b82a1
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

Fn/FnOnceの呼び出し回数契約だけを扱う。非同期クロージャは扱わない。

## References

[1] [Rust Error Codes: E0525](https://doc.rust-lang.org/error_codes/E0525.html)
