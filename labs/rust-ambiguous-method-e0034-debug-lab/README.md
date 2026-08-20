# Rust E0034: 同名メソッドの呼び出し曖昧性

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

Taskの短いタイトルを選んで返す。

## 最短の開始手順

```bash
cargo check
cargo test
```

## バグを再現する

バグ状態コミット`63b4b60`では、`cargo check`が終了コード`101`でE0034を報告します。修正は`5617612`です。

```bash
git switch --detach 63b4b60
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

複数トレイトの同名メソッドを完全修飾構文で選ぶことだけを扱う。

## References

[1] [Rust Error Codes: E0034](https://doc.rust-lang.org/error_codes/E0034.html)
