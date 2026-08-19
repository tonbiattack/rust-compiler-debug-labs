# Rust Compiler Debug Labs

Rustコンパイラ診断を、**再現 → 観測 → 最小修正 → 回帰**の流れで学ぶための実行可能な教材集です。各ラボは外部依存を使わない独立したCargoプロジェクトであり、バグ時・修正時のCargo出力、`rustc --explain`、統合テスト、調査記録を含みます。

## 含まれる成果物

各ラボは`labs/`配下にあり、以下を含みます。

| パス | 内容 |
| --- | --- |
| `src/lib.rs` | 修正済みの最小実装 |
| `tests/` | 公開APIの契約を固定する統合テスト |
| `docs/observed-*.txt` | バグ状態・修正状態で観測したCargo出力 |
| `docs/rustc-explain-*.txt` | 対象診断の公式コンパイラ説明 |
| `README.md` | 学習対象、バグ状態、最短の実行手順 |
| `docs/debugging-record.md` | 仮説、原因、最小修正、回帰保証 |

## 最短の実行手順

任意のラボへ移動し、Cargoテストを実行します。

```bash
cd labs/rust-orphan-rule-e0117-debug-lab
cargo test
```

すべてのラボを検証するには、`scripts/test-all.sh`を実行します。

```bash
bash scripts/test-all.sh
```

## バグ状態を学ぶ方法

各ラボの`docs/observed-cargo-check-bug.txt`には実際の診断を保存しています。個別の再現コミットは、元のコンテンツリポジトリで管理されています。このリポジトリは、**修正済みで実行可能な教材コードと検証資料**をまとめた配布用の構成です。Qiita投稿用下書きは含めません。

## カタログ

収録診断とラボの対応は[CATALOG.md](CATALOG.md)を参照してください。
