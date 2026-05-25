# Changelog

All notable changes to this project will be documented in this file.
## [0.2.0] - 2026-05-25

### 🚀 Features

- ID のゼロ埋めを廃止し整数 ID に変更する

### 📚 Documentation

- Issues/ 探索ルールを README に追記する

### 🧪 Testing

- サブディレクトリからの issues/ 探索テストを追加する
## [0.1.0] - 2026-05-24

### 🚀 Features

- Bin/ スクリプト追加・スキル名を fbim に変更・README を拡充する
- Skills/fbim/scripts を bin/ への symlink にしてスクリプト呼び出しを整理する
- Bin/fbim CLI を追加する
- Fbim に help サブコマンドと create --body オプションを追加する
- Kiwi 固有の設定を除去し公開可能な状態にする
- シェル completion を追加し SKILL.md に argument-hint を付ける
- Issue ID を5桁に変更し4桁との後方互換を維持する
- 個別スキル fbim-create/done/pending/reopen/list/show を追加する
- 親ディレクトリ探索・issues_dir 設定・補完のタイトル表示を追加する
- サブプロセスカバレッジを設定する（88%）
- Python/bash 実装を Rust バイナリに全面書き直し
- Package Registry を使ったバイナリ配布を整備する
- カバレッジ計測・doctest・README を整備する
- Completions サブコマンド・統合テスト・ソース整理
- コールバック方式の動的シェル補完を実装する
- Fbim init コマンドの追加と README の改善
- Python/bash 実装を Rust バイナリに全面移行する

### 🐛 Bug Fixes

- ヘルプ文字列・ドキュメントの残課題を修正する
- CI でカバレッジが収集されない問題を修正する
- Coverage run を明示的に使いKubernetes CI でのカバレッジ収集を修正する
- Install.sh のアーティファクト URL を GitLab API エンドポイントに修正する
- Install.sh が glab のトークンを自動で使うよう修正する
- スキルを Rust バイナリに対応させる
- Completions の broken pipe パニックを修正する
- Zsh 補完スクリプトに compdef _fbim fbim を追加する
- Bash 補完を source <(...) から eval "$(...)" に変更する

### 💼 Other

- FBIM リポジトリの初期構成を作成する
- 英語を正とし日本語版を .ja.md として追加する

### 🚜 Refactor

- SKILL.md を bin/fbim 呼び出しに簡略化する

### 📚 Documentation

- Issue ID の表記を5桁に統一する
- 公開準備として README・LICENSE・スキルを整備する
- README.ja.md を Rust 版・英語版に合わせて全面改訂
- Completions のヘルプと README にシェル補完の使い方を追加

### 🎨 Styling

- Cargo fmt を適用する（CLI コマンド属性の折り返し）

### 🧪 Testing

- Bin/ スクリプトの pytest テストスイートを追加する
- 親ディレクトリ探索・issues_dir 設定のテストを追加する

### ⚙️ Miscellaneous Tasks

- .gitignore を追加して __pycache__ を除外する
- GitLab CI で pytest を実行する
- カバレッジを GitLab に報告しバッジを README に追加する
- Rustfmt と clippy コンポーネントを CI に追加する
- Rust-toolchain.toml で 1.95.0 を固定し rustfmt・clippy を宣言する
- Build ジョブを全ブランチで実行するよう修正する
- Publish ジョブのエラー出力を有効化して原因を確認する
- Publish の curl に --insecure を追加して自己署名証明書に対応する
