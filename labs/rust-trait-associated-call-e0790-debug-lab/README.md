# Rust E0790: 実装型を伴わない関連関数呼び出し

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

既定方針として`News`実装のプレフィックス`news`を返す。関連関数の呼び出し時に実装型を明示する。

## 最短の開始手順

```bash
cargo check
cargo test
```

修正済みのmainブランチでは、両コマンドが成功します。

## バグを再現する

バグ状態コミット`d7facb4`では、`cargo check`が終了コード`101`でE0790を報告します。

```bash
git switch --detach d7facb4
cargo check
git switch main
```

最小修正はコミット`b329f86`です。

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 最小実装 |
| `tests/` | 公開APIの契約テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測・修正・回帰の記録 |

## スコープ

トレイト関連関数の実装選択だけを扱い、メソッド解決全般は扱わない。

## References

[1] [Rust Error Codes: E0790](https://doc.rust-lang.org/error_codes/E0790.html)
