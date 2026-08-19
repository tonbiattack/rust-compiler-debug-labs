# Rust E0432: 解決できないimportパス

Rust/Cargo 1.75系で検証した、モジュールの名前解決を扱う基礎ラボです。外部依存はありません。

## この題材で守る契約

`status()`は`labels::format`を利用して、`status:ready`を返します。importは実在する子モジュールを指します。

## 最短の開始手順

```bash
cargo check
cargo test
```

修正済みの既定ブランチでは、上記が成功します。

## バグを再現する

バグ状態コミット`d29ee83`へ切り替えると、`cargo check`は終了コード`101`でE0432を出します。

```bash
git switch --detach d29ee83
cargo check
```

修正はコミット`d197eb8`です。元の作業ブランチへ戻るには`git switch main`を実行します。

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 子モジュールと公開関数 |
| `tests/status.rs` | 整形結果の契約テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測から回帰確認までの記録 |

## スコープ

このラボは現行モジュールの子を`use`するパスだけを扱います。外部クレートやRust 2015の`extern crate`は扱いません。

## References

[1] [Rust Error Codes: E0432](https://doc.rust-lang.org/error_codes/E0432.html)
