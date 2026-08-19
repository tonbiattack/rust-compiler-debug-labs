# Rust E0038: dyn互換でないトレイト

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

トレイトオブジェクト越しに識別子`n-1`を描画する。`Self`を返す複製操作は動的ディスパッチの対象から除外する。

## 最短の開始手順

```bash
cargo check
cargo test
```

修正済みのmainブランチでは、両コマンドが成功します。

## バグを再現する

バグ状態コミット`7ec7cbe`では、`cargo check`が終了コード`101`でE0038を報告します。

```bash
git switch --detach 7ec7cbe
cargo check
git switch main
```

最小修正はコミット`d394c31`です。

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 最小実装 |
| `tests/` | 公開APIの契約テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測・修正・回帰の記録 |

## スコープ

dyn互換性のうち`Self`を戻り値に持つメソッドだけを扱う。ジェネリックメソッド等の他条件は扱わない。

## References

[1] [Rust Error Codes: E0038](https://doc.rust-lang.org/error_codes/E0038.html)
