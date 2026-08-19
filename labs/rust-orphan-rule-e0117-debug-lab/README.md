# Rust E0117: 孤児ルール

Rust/Cargo 1.75系、外部依存なしで検証したデバッグラボです。

## この題材で守る契約

ラベル集合を表示用文字列`rust,cargo`へ変換する。外部型へ外部トレイトを実装せず、ローカル型を境界にする。

## 最短の開始手順

```bash
cargo check
cargo test
```

修正済みのmainブランチでは、両コマンドが成功します。

## バグを再現する

バグ状態コミット`65b7b22`では、`cargo check`が終了コード`101`でE0117を報告します。

```bash
git switch --detach 65b7b22
cargo check
git switch main
```

最小修正はコミット`0389acc`です。

## 構成

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 最小実装 |
| `tests/` | 公開APIの契約テスト |
| `docs/observed-*.txt` | バグ時・修正時のCargo出力 |
| `docs/debugging-record.md` | 観測・修正・回帰の記録 |

## スコープ

コヒーレンス（孤児ルール）だけを扱い、表示形式の設計一般は扱わない。

## References

[1] [Rust Error Codes: E0117](https://doc.rust-lang.org/error_codes/E0117.html)
