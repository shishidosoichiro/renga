# Changelog

All notable changes to this project will be documented in this file.
## [0.6.0] - 2026-05-28

### 🚀 Features

- [`0a80332`] *(cli)* Renga edit・renga update コマンドを追加 (#86, #88)
## [0.5.1] - 2026-05-28

### 🚀 Features

- [`ad18af6`] Crates.io に publish (#57) — バッジ追加・cargo install renga に更新
## [0.5.0] - 2026-05-28

### 🚀 Features

- [`ad0b6fc`] *(cli)* --body - で標準入力から本文を読む機能と --id でイシュー番号を指定する機能を追加 (#50, #51)
- [`4a384eb`] *(core)* Schema_version フィールドをフロントマターに追加 (#65)
- [`78d9e53`] *(cli)* Fbim validate コマンドを追加 (#64)
- [`2348004`] [**breaking**] ツール名を fbim から renga に変更 (#83)
- [`d73b9d0`] Vercel-labs/skills 対応 — npx skills add でインストール可能に

### 🐛 Bug Fixes

- [`9f026e7`] *(core)* Set_frontmatter_field が body の '---' で frontmatter を再解析するバグを修正する (#18, #41)
- [`ce40887`] *(core)* Reopen/done/create のファイル操作安全性を改善する (#19, #20, #21, #45, #47, #48, #49)
- [`1d80d02`] *(config)* .fbim.yml の YAML パースエラーを握り潰さずエラーを返す (#22)
- [`7434c36`] *(core,cli)* Make_slug の末尾ダッシュを除去し --area デフォルト値を空文字に修正する (#25, #28)
- [`6575e26`] *(cli)* --status 不正値エラー化・補完修正・help 更新 (#23, #24, #26, #27, #31, #35)
- [`89491bd`] *(core)* タイトル抽出でファイルステムへのフォールバックを実装 (#34)
- [`8d71b4e`] *(core)* --id 0 と非数値を弾くバリデーション追加、spec に --id と --body - を記載 (#52, #53, #54)
- [`78614e5`] *(core)* Reopen が open な issue を上書きできてしまうバグを修正

### 🚜 Refactor

- [`e4aa4f6`] Retro を notes/ ファイルから issue に移行し self-improve を更新する
## [0.4.0] - 2026-05-26

### 🚀 Features

- [`cfb6ff4`] *(core)* Issue に milestone フィールドを追加する

### 🐛 Bug Fixes

- [`d0b951b`] Area デフォルトを空文字に統一し Regex を LazyLock 化する
## [0.3.0] - 2026-05-26

### 🚀 Features

- [`016446f`] Frontmatter のないファイルの status を unknown にする
- [`382e339`] Issues/ を再帰的に検索し Priority::Unknown を追加する

### 🐛 Bug Fixes

- [`4f62bf8`] Parse できない issue ファイルをスキップして警告を出す
- [`2f3463d`] Frontmatter のない Markdown ファイルもデフォルト値で表示する
## [0.2.1] - 2026-05-25

### 🐛 Bug Fixes

- [`dd476c5`] Fbim -- のタブ補完で --version と --help を表示する
- [`4414c1e`] List の並び順を数値順に修正する
## [0.2.0] - 2026-05-25

### 🚀 Features

- [`03c12c7`] ID のゼロ埋めを廃止し整数 ID に変更する
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

### 🚜 Refactor

- [`0c67786`] SKILL.md を bin/fbim 呼び出しに簡略化する
