# E0373 調査記録：`thread::spawn` のクロージャ捕捉

## 目的

Rust 1.75.0 で、`thread::spawn` に渡したクロージャが関数ローカルの `String` を参照として捕捉すると、完了メッセージを返す前に E0373 が発生する理由を、最小例で確認します。

> 契約：`complete_in_worker("月次集計".to_owned())` に対して `"月次集計: 完了"` を得ます。バグ状態では、コンパイラが E0373 を出してビルドを停止します。

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 言語処理系 | Rust 1.75.0 (`rustc 1.75.0`) |
| 難易度プロファイル | 実践・上級。クロージャの既定の捕捉方法と、別スレッドへ渡す `thread::spawn` の `'static` 制約を区別して読む必要があります。 |
| ビルド・テスト方法 | `cargo check`、`cargo test`、`cargo run` |
| 使用する依存関係 | Rust標準ライブラリのみ |
| 使用しないもの | Webフレームワーク、外部サービス、asyncランタイム、共有可変状態 |
| 公開境界 | `complete_in_worker(label: String) -> String` |
| 最終観測 | 関数の戻り値を統合テストで直接確認します。 |
| 決定性の確保 | 固定された入力を使い、`JoinHandle::join` でワーカースレッド終了を待ちます。`sleep` は使用しません。 |

この境界を選んだ理由は、クロージャ捕捉と `thread::spawn` の契約を、フレームワークを介さずにコンパイラ診断と関数の戻り値で直接観測できるためです。

## 最初に観測した事実

| 観測順 | 事実 | 得られた証拠 |
| --- | --- | --- |
| 1 | `complete_in_worker` は `String` を受け取り、クロージャ内の `format!` で `label` を使っていました。 | `src/lib.rs` と `tests/worker_completion.rs` |
| 2 | `cargo check` は終了コード 101 で失敗しました。 | `docs/observed-cargo-check-bug.txt` |
| 3 | 診断は `label` を借用しているクロージャが現在の関数より長生きする可能性を示し、E0373 を割り当てました。 | 同ファイルの `error[E0373]`、`label is borrowed here`、`function requires argument type to outlive 'static` |
| 4 | `handle.join()` はバグ状態ですでに呼んでいましたが、コンパイルは受理されませんでした。 | `src/lib.rs` と `docs/observed-cargo-check-bug.txt` |
| 5 | `cargo test` も、テストのアサーションに到達する前に同じ E0373 で終了コード 101 になりました。 | `docs/observed-cargo-test-bug.txt` |

バグ状態のコミットは `d4aeb85` です。`cargo check` を実行すると、設定や依存解決ではなく、意図した E0373 だけを確認できます。

診断の中心部分は次のとおりです。行番号はバグ状態の `src/lib.rs` を指します。

```text
error[E0373]: closure may outlive the current function, but it borrows `label`, which is owned by the current function
 --> src/lib.rs:5:32
  |
5 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `label`
6 |         format!("{label}: 完了")
  |                   ----- `label` is borrowed here
  |
note: function requires argument type to outlive `'static`
help: to force the closure to take ownership of `label` ..., use the `move` keyword
```

## 競合仮説と検証

| 仮説 | 予測 | 検証 | 結果 |
| --- | --- | --- | --- |
| `label` が参照捕捉され、クロージャが `'static` を満たせない | 診断に借用箇所と `'static` 要求が表示される | `cargo check` の全文を保存して確認する | 支持。診断は `label` の借用、`thread::spawn` の `'static` 要求、`move` の提案を同時に示しました。 |
| `join` を呼ばないことが原因である | `join` を追加すればコンパイルが通る | バグ状態で `handle.join()` を残したまま `cargo check` を実行する | 棄却。`join` があっても E0373 になりました。公式E0373説明も、`join` があってもコンパイラが安全性を証明できないため受理しないと説明します。[1] |
| スレッド処理では `String` を使えない | `move` を追加しても失敗する | `thread::spawn(move || ...)` にだけ変更して同じ統合テストを実行する | 棄却。`cargo check`、`cargo test`、`cargo run` が成功しました。 |

## 確定した原因

`thread::spawn` のシグネチャは、渡すクロージャ `F` に `FnOnce() -> T + Send + 'static` を要求します。非スコープ化スレッドは呼び出し元より長く実行され得るため、実行元のスタック上にある値への参照を保持したクロージャを受け取れません。[2]

バグ状態の `||` は、`format!` が `label` を読むだけなので、既定で `label` を参照として捕捉します。その参照の元である `label` は `complete_in_worker` のローカル変数であり、関数から戻れば存在しません。コンパイラは「スレッドが関数より後まで生きる可能性」を排除できないため、参照を含むクロージャを拒否します。E0373の公式説明も、クロージャの参照捕捉によって元の値が存在しなくなり得る場合にこのエラーが起き、スレッド生成が代表例であると説明しています。[1]

この問題の安全性上の意味は、スレッドが保持する参照先を親スレッドが先に破棄する状況を許可しないことです。`join` は実行時の待機操作ですが、`thread::spawn` のAPIは一般にスレッドを切り離せるため、コンパイラはクロージャを渡す時点で `'static` という契約を満たすか検査します。[2]

## 最小修正

変更はクロージャの前に `move` を追加する一行だけです。

```rust
let handle = thread::spawn(move || {
    format!("{label}: 完了")
});
```

`move` はクロージャが使う `label` の所有権をクロージャ環境へ移します。したがって、クロージャは呼び出し元のスタック上の `label` への参照を保持しません。Rust公式Bookも、`thread::spawn` に渡すクロージャでは、環境の値の所有権を新しいスレッドへ移すために `move` を使うことが多いと説明しています。[3]

この修正は参照捕捉と `'static` の不整合だけを対象にしています。APIの引数型を変えず、`Arc`、`Mutex`、チャネル、外部依存、スレッドプールを追加していません。修正コミットは `dc5c0e3` です。

## 別の修正方法とトレードオフ

このラボの契約では、処理名をワーカースレッドだけが使うため、`move` が最小かつ自然な修正です。呼び出し元でも同じ文字列を引き続き使う必要がある場合は、所有権を移す前に複製する方法があります。

```rust
let worker_label = label.clone();
let handle = thread::spawn(move || format!("{worker_label}: 完了"));
println!("開始: {label}");
```

`clone` は独立した `String` のヒープ割り当てとバイト列コピーを伴いますが、呼び出し元とワーカースレッドの双方が独立して所有できます。共有可変状態が必要なときの `Arc<Mutex<T>>` は別の問題を解く手段であり、このような読み取り専用の一回限りの値渡しに導入する必要はありません。

| 選択肢 | 所有権 | コスト | 適用条件 |
| --- | --- | --- | --- |
| `move` | 値をワーカースレッドへ移す | 追加コピーなし | 呼び出し元が値を以後使わない |
| `clone` して `move` | 呼び出し元とワーカーが別々の値を所有する | 文字列の割り当て・コピー | 呼び出し元でも値を使い続ける |
| `Arc` 等で共有 | 複数の所有者で共有する | 参照カウント、設計・同期の複雑さ | 複数箇所で長期的に共有する必要がある |

## 回帰保証

| 守ること | テストまたは診断 | 修正後の結果 |
| --- | --- | --- |
| ワーカーが完了メッセージを返す | `returns_a_completion_message_from_the_worker` | `"月次集計: 完了"` を返して成功 |
| メッセージ形式が空文字列でも一貫する | `accepts_an_empty_label_without_changing_the_message_format` | `": 完了"` を返して成功 |
| 実行可能な状態を保つ | `cargo check`、`cargo test`、`cargo run` | すべて終了コード 0。統合テスト2件が成功し、実行出力は `月次集計: 完了` |

固定済みの状態で `cargo test` を実行し、統合テスト2件の成功を確認しました。出力は `docs/observed-cargo-test-fixed.txt` に保存しています。

## 再現手順

```bash
# 修正済み状態を検証する
cargo test
cargo run

# バグ状態を確認する。作業中の変更は先に退避する
git switch --detach d4aeb85
cargo check

# 修正済み状態へ戻る
git switch main
```

## スコープと注意点

このラボで確認したのは、Rust 1.75.0 の `std::thread::spawn` に、関数ローカルの `String` を参照捕捉するクロージャを渡した条件です。スコープ付きスレッドは異なるAPI契約を持ち、asyncブロック、`Send` 制約、共有可変状態、性能、別のRustバージョンへ同じ結論を自動的に広げるものではありません。

## References

[1]: https://doc.rust-lang.org/error_codes/E0373.html "Rust error code E0373"
[2]: https://doc.rust-lang.org/std/thread/fn.spawn.html "std::thread::spawn"
[3]: https://doc.rust-lang.org/book/ch16-01-threads.html "The Rust Programming Language: Using Threads to Run Code Simultaneously"
