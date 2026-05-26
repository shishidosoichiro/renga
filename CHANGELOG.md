# Changelog

All notable changes to this project will be documented in this file.
## [0.4.0] - 2026-05-26

### 🚀 Features

- [`cfb6ff4`] *(core)* Issue に milestone フィールドを追加する

### 🐛 Bug Fixes

- [`d0b951b`] Area デフォルトを空文字に統一し Regex を LazyLock 化する

### 📚 Documentation

- [`a388c5f`] CLAUDE.md にリリース手順と git-cliff の使用を明記する
- [`e1e7c6c`] Install・release スキルを追加し CONTRIBUTING.md を CLAUDE.md にインポートする
- [`f1aaa3f`] Self-improve・review サブエージェントを追加する
- [`041cb4a`] CLAUDE.md を63行に削減する
- [`a5008a2`] CLAUDE.md の ID 仕様を整数 ID に修正する（ゼロ埋め廃止済み）
- [`e04b4ce`] *(issues)* Issue 7〜16 の状態を更新する
- [`521ba36`] 実装フローとカバレッジ確認手順を CLAUDE.md・CONTRIBUTING.md に追加する
- [`9229c50`] Review サブエージェントの観点にカバレッジ確認を明示する
- [`9d5ec5b`] エージェント活用方針と .claude/ 変更ルールを CLAUDE.md に追加し review にカバレッジ確認を加える
- [`c916c1a`] *(claude)* Retro の指摘を CLAUDE.md に反映する

### ⚙️ Miscellaneous Tasks

- [`1be9807`] Skills を .claude/skills に移動し CLAUDE.md を更新する
## [0.3.0] - 2026-05-26

### 🚀 Features

- [`016446f`] Frontmatter のないファイルの status を unknown にする
- [`382e339`] Issues/ を再帰的に検索し Priority::Unknown を追加する

### 🐛 Bug Fixes

- [`4f62bf8`] Parse できない issue ファイルをスキップして警告を出す
- [`2f3463d`] Frontmatter のない Markdown ファイルもデフォルト値で表示する

### 📚 Documentation

- [`b07f22e`] ID とタイトルの取得元を spec と doc コメントに明記する
- [`2a4a9f8`] Spec.md を Rust 版に更新し、CLAUDE.md にドキュメント更新ルールを追記する
- [`be0b7d4`] Unknown status を spec と README に追記する
- [`d1027ed`] Status::Unknown の doc コメントと doctest を追加する
## [0.2.1] - 2026-05-25

### 🐛 Bug Fixes

- [`dd476c5`] Fbim -- のタブ補完で --version と --help を表示する
- [`4414c1e`] List の並び順を数値順に修正する

### 📚 Documentation

- [`fad6ea7`] CONTRIBUTING.md を追加する
- [`28f9876`] README のキャッチコピーを修正する
- [`78c3dde`] README の説明文を改善する
- [`32521ea`] README を全面的に改善する
- [`dc545ea`] "Why file-based?" セクションのフォーマットを戻す
- [`72a4cf3`] AI ツールとの連携を "Why file-based?" セクションに戻す

### ⚙️ Miscellaneous Tasks

- [`252e740`] .coverage と __pycache__/ を .gitignore に追加する
- [`242741b`] CHANGELOG にコミット ID を追加する
## [0.2.0] - 2026-05-25

### 🚀 Features

- [`03c12c7`] ID のゼロ埋めを廃止し整数 ID に変更する

### 📚 Documentation

- [`4e2865b`] Issues/ 探索ルールを README に追記する

### 🧪 Testing

- [`1b6ef36`] サブディレクトリからの issues/ 探索テストを追加する
## [0.1.0] - 2026-05-24

### 🚀 Features

- [`98532d0`] Bin/ スクリプト追加・スキル名を fbim に変更・README を拡充する
- [`b4f25b4`] Skills/fbim/scripts を bin/ への symlink にしてスクリプト呼び出しを整理する
- [`8cc5e5a`] Bin/fbim CLI を追加する
- [`935ee07`] Fbim に help サブコマンドと create --body オプションを追加する
- [`1d6c6ac`] Kiwi 固有の設定を除去し公開可能な状態にする
- [`e79dff7`] シェル completion を追加し SKILL.md に argument-hint を付ける
- [`76c29b7`] Issue ID を5桁に変更し4桁との後方互換を維持する
- [`7b46007`] 個別スキル fbim-create/done/pending/reopen/list/show を追加する
- [`865d504`] 親ディレクトリ探索・issues_dir 設定・補完のタイトル表示を追加する
- [`035025b`] サブプロセスカバレッジを設定する（88%）
- [`76ba77e`] Python/bash 実装を Rust バイナリに全面書き直し
- [`9c2afe9`] Package Registry を使ったバイナリ配布を整備する
- [`ca5fc13`] カバレッジ計測・doctest・README を整備する
- [`4f4fa3c`] Completions サブコマンド・統合テスト・ソース整理
- [`c4d57ed`] コールバック方式の動的シェル補完を実装する
- [`f55dea5`] Fbim init コマンドの追加と README の改善
- [`a6a16d2`] Python/bash 実装を Rust バイナリに全面移行する

### 🐛 Bug Fixes

- [`227f988`] ヘルプ文字列・ドキュメントの残課題を修正する
- [`ef077a3`] CI でカバレッジが収集されない問題を修正する
- [`fe2dbdd`] Coverage run を明示的に使いKubernetes CI でのカバレッジ収集を修正する
- [`85f3ccf`] Install.sh のアーティファクト URL を GitLab API エンドポイントに修正する
- [`f0e9410`] Install.sh が glab のトークンを自動で使うよう修正する
- [`5b7425c`] スキルを Rust バイナリに対応させる
- [`7640f29`] Completions の broken pipe パニックを修正する
- [`ee4c2f5`] Zsh 補完スクリプトに compdef _fbim fbim を追加する
- [`ad040df`] Bash 補完を source <(...) から eval "$(...)" に変更する

### 💼 Other

- [`eabf8b1`] FBIM リポジトリの初期構成を作成する
- [`2fe479d`] 英語を正とし日本語版を .ja.md として追加する

### 🚜 Refactor

- [`0c67786`] SKILL.md を bin/fbim 呼び出しに簡略化する

### 📚 Documentation

- [`812c9bd`] Issue ID の表記を5桁に統一する
- [`c35b47b`] 公開準備として README・LICENSE・スキルを整備する
- [`30c4091`] README.ja.md を Rust 版・英語版に合わせて全面改訂
- [`3e3eaa1`] Completions のヘルプと README にシェル補完の使い方を追加

### 🎨 Styling

- [`564faec`] Cargo fmt を適用する（CLI コマンド属性の折り返し）

### 🧪 Testing

- [`15c2bdb`] Bin/ スクリプトの pytest テストスイートを追加する
- [`0689431`] 親ディレクトリ探索・issues_dir 設定のテストを追加する

### ⚙️ Miscellaneous Tasks

- [`e0e6208`] .gitignore を追加して __pycache__ を除外する
- [`a00fe63`] GitLab CI で pytest を実行する
- [`96783fc`] カバレッジを GitLab に報告しバッジを README に追加する
- [`1729425`] Rustfmt と clippy コンポーネントを CI に追加する
- [`49a7f57`] Rust-toolchain.toml で 1.95.0 を固定し rustfmt・clippy を宣言する
- [`2261ddf`] Build ジョブを全ブランチで実行するよう修正する
- [`20db4ff`] Publish ジョブのエラー出力を有効化して原因を確認する
- [`f2680ac`] Publish の curl に --insecure を追加して自己署名証明書に対応する
