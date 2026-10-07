---
paths:
  - "issues/**/*.md"
---

# Issue 管理ルール

## area の選択

| area | 使うとき |
|---|---|
| `cli` | コマンドライン引数・ヘルプ表示 |
| `core` | issue のパース・ファイル操作・プロジェクトルート探索 |
| `config` | `.renga.yml` の読み込み・設定 |
| `test` | テストの追加・修正 |
| `docs` | ドキュメント・README・CONTRIBUTING |
| `ci` | CI/CD パイプライン |
| `agent` | CLAUDE.md・`.claude/agents/` の変更・retro issue |
| `misc` | 上記に当てはまらないもの |

## バグ issue のラベル規約

レビューで見つかったバグを issue 化するときは必ず以下のラベルを付ける。

| ラベル | 意味 | コミット戦略 |
|---|---|---|
| `found_in_impl` | 今の実装サイクルで入ったバグ | コミット前に修正し feature コミットに含める |
| `found_at:X.Y.Z` | 指定バージョンから存在していたバグ | コミット後に別の `fix:` コミットで修正する |
| `fixed_at:X.Y.Z` | 修正されたバージョン | クローズ時に付ける |

## 判断記録の出典（retro #268）

Renga は公開リポジトリなので、issue 本文に書く根拠・出典は公開リポジトリの読み手が辿れるもの（Renga 自身の issue・コード・公開された一次資料）に限る。親ディレクトリなど作業ディレクトリ外の非公開プロジェクトの名前・パス・issue 番号は書かず、得た知見はユースケースか一般論として書く。

## retro issue の起票ルール

自己改善のための retro issue は以下のフォーマットで起票する。

```sh
renga create "retro: <内容>" --area agent --label retro
```

- `area: agent`、`labels: [retro]` を必ず付ける
- 起票後に `Agent(subagent_type="self-improve")` を呼んで改善を実施する
