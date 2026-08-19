# `thread::spawn` の E0373 を読むデバッグラボ

Rust の `thread::spawn` に渡すクロージャがローカル変数を参照捕捉したときに発生する **E0373** を、**失敗する統合テスト → コンパイラ診断の確認 → 最小修正 → 回帰テスト**の順に学ぶ、標準ライブラリだけのデバッグ教材です。

## この題材で守る契約

> `complete_in_worker` に処理名の `String` を渡したとき、ワーカースレッドが `<処理名>: 完了` を返します。

バグ状態では、クロージャが `label` を参照として捕捉するため、`thread::spawn` が求める `'static` 制約を満たせず **E0373** でコンパイルに失敗します。特定のWebフレームワーク、外部サービス、非同期ランタイムは使用しません。

## 最短の開始手順

修正済みの状態で、次を実行します。

```bash
cargo test
cargo run
```

`cargo test` は統合テスト2件を成功させ、`cargo run` は次を出力します。

```text
月次集計: 完了
```

## バグを再現する

バグ状態はコミット `d4aeb85` に保存しています。作業中の変更を退避してから、次を実行します。

```bash
git switch --detach d4aeb85
cargo check
```

`closure may outlive the current function, but it borrows 'label'` を含む **E0373** が表示されます。`handle.join()` がコード内にあっても、コンパイラはこの借用を受理しません。確認後は修正済み状態へ戻ります。

```bash
git switch main
cargo test
```

## 観測の要約

| 観測点 | バグ状態 | 修正後 |
| --- | --- | --- |
| クロージャの `label` 捕捉 | 参照捕捉 | `move` により所有権を取得 |
| コンパイル | E0373で失敗 | `cargo check` が成功 |
| 利用者視点の結果 | 結果を返す前にコンパイル停止 | `"月次集計: 完了"` を返す |
| 境界ケース | 検証できない | 空文字列でも `": 完了"` を返す |
| 全体検証 | `cargo test` がE0373で失敗 | 2件の統合テストが成功 |

詳しい観測、仮説の比較、原因、修正、回帰保証は [docs/debugging-record.md](docs/debugging-record.md) に記録しています。

## 構成

```text
src/lib.rs                         最小再現と修正済み実装
src/main.rs                        実行用エントリーポイント
tests/worker_completion.rs         利用者視点の統合テスト
docs/topic-brief.md                題材の選定と再現設計
docs/debugging-record.md           観測・仮説・原因・修正の記録
docs/observed-*.txt                実際のコンパイラとテストの出力
```

## 前提条件

| 項目 | バージョンまたは条件 |
| --- | --- |
| Rust | `rustc 1.75.0` |
| Cargo | `cargo 1.75.0` |
| 依存関係 | Rust標準ライブラリのみ |
| 外部サービス | 不要 |

## スコープ

このラボは、非スコープ化スレッドである `std::thread::spawn` に渡すクロージャが、関数ローカルの `String` を参照捕捉したときの **E0373** だけを扱います。`Arc` や `Mutex` による共有可変状態、スコープ付きスレッド、async、スレッド数の設計、性能評価は扱いません。

## References

[1]: https://doc.rust-lang.org/error_codes/E0373.html "Rust error code E0373"
[2]: https://doc.rust-lang.org/std/thread/fn.spawn.html "std::thread::spawn"
[3]: https://doc.rust-lang.org/book/ch16-01-threads.html "The Rust Programming Language: Using Threads to Run Code Simultaneously"
