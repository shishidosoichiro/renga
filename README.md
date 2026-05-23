# FBIM — File-Based Issue Management

ファイルベースのイシュー管理システム。番号付きファイルで issue を管理し、Claude Code スキルから操作する。

## 概要

- issue は `NNNN-short-name.md` 形式のファイルで管理する
- `issues/` 配下に open/pending、`issues/done/` 配下に完了分を置く
- 詳細は [spec.md](spec.md) を参照

## インストール

Claude Code スキルとして使うにはシンボリックリンクを張る。

```sh
ln -s ~/kiwi/fbim/skills/issue ~/.claude/skills/issue
```

インストール後は `/issue` で呼び出せる。

## 使い方

| コマンド | 動作 |
|---|---|
| `/issue タイトル` | issue を作成する（デフォルト） |
| `/issue create タイトル` | 同上（明示形） |
| `/issue done NNNN` | issue を done に移動する |
| `/issue pending NNNN` | issue を pending にする |
| `/issue reopen NNNN` | done の issue を open に戻す |

`issues/README.md`（一覧）は各コマンドの後に自動再生成される。

## ファイルの更新・閲覧

issue の本文更新は対象ファイルを直接編集する。閲覧も同様（Read ツールや cat で開く）。
