# E0184 調査記録

## 実行環境と再現境界

Rust/Cargo 1.75系で、`Ticket`のderiveとDrop実装を最小境界にした。

## 最初に観測した事実

バグコミット`704d604`で`cargo check`と`cargo test`は終了コード`101`となり、`error[E0184]: the trait Copy cannot be implemented for this type; the type has a destructor`を出力した。

## 競合仮説と検証

| 仮説 | 予測 | 最小実験 | 結果 | 判定 |
| --- | --- | --- | --- | --- |
| フィールド型がCopyでない | deriveを確認 | idはu32 | 直接原因ではない | 棄却 |
| DropとCopyが両立しない | Copyを外す | cargo check成功 | 診断消失 | 採用 |

## 確定した原因

Dropを実装する型はCopyを実装できない。[1]

## 最小修正

Copyを外してCloneをderiveし、`duplicate`内で`ticket.clone()`を明示した。

## 回帰保証

`tests/duplicate.rs`が2つのチケットのidを検証する。成功出力は`observed-cargo-*-fixed.txt`に保存した。

## 再現手順

`git switch --detach 704d604 && cargo check`でE0184を確認し、`git switch main && cargo test`で修正状態を確認する。

## スコープと注意点

このラボはE0184の直接原因だけを扱う。

## References

[1] [Rust Error Codes: E0184](https://doc.rust-lang.org/error_codes/E0184.html)
