# Rust E0407: トレイト実装に余分なメソッドを置く

Rust/Cargo 1.75系で検証した、トレイト契約と型固有APIの境界を扱う基礎ラボです。外部依存はありません。

## この題材で守る契約

`Task`は`Renderer`として描画でき、型固有の`display_name()`も提供します。後者は`Renderer`の契約には含めません。

## 最短の開始手順

```bash
cargo check
cargo test
```

修正済みの既定ブランチでは、上記が成功します。

## バグを再現する

バグ状態コミット`189b5c0`へ切り替えると、`cargo check`は終了コード`101`でE0407を出します。

```bash
git switch --detach 189b5c0
cargo check
```

修正はコミット`95b65c5`です。元の作業ブランチへ戻るには`git switch main`を実行します。

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | トレイトと最小実装 |
| `tests/renderer.rs` | 描画・表示名の契約テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測から回帰確認までの記録 |

## スコープ

このラボはトレイト実装内に定義できるメソッドだけを扱います。トレイトへメソッドを追加するAPI設計の判断は扱いません。

## References

[1] [Rust Error Codes: E0407](https://doc.rust-lang.org/error_codes/E0407.html)
