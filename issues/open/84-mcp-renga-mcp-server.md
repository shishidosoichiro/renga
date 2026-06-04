---
status: open
priority: low
area: misc
labels: []
---

# MCP サーバーモード（renga mcp-server）の追加を検討する

## 背景

renga の差別化ポイント「AI エージェントが直接操作できる」をより本物にするため、MCP（Model Context Protocol）サーバーとして起動できるモードの追加を検討する。現在はスキル（SKILL.md）経由で Claude がシェルコマンドを自分で実行する方式。

## MCP にすることで得られるもの

- Claude Desktop・Cursor など Claude Code 以外のクライアントからも使える
- ツールが JSON Schema で型定義される → Claude がより確実に呼び出せる
- 将来的に複数エージェントが協調して issue を管理できる

## 実装方法

- `rmcp` クレート（公式 Rust MCP SDK）を使う
- `renga mcp-server` サブコマンドとして stdio transport で起動
- Claude Code への登録は `settings.json` の `mcpServers` に追加

```json
{
  "mcpServers": {
    "renga": {
      "command": "/usr/local/bin/renga",
      "args": ["mcp-server"]
    }
  }
}
```

## rmcp の現状（2026-06-04 調査）

- v0.16.x、470 万 DL。十分に成熟しており安定性の懸念は解消
- 1.x へのマイグレーションが進行中（breaking changes あり）。0.16.x で実装して 1.x 対応は後追いにするか、1.x リリースを待つか判断が必要
- 既存の `renga::issue`・`renga::config` ライブラリコードはそのまま使える（シェルアウト不要）

## プロジェクトルート問題

CLI は起動時にディレクトリを遡って自動検出するが、MCP サーバーは**一度起動したら複数のツール呼び出しを処理し続ける**ため、どのプロジェクトを操作するかを明示する必要がある。

推奨：起動時に `--root` を渡す方式（sqlite MCP と同じパターン）

```json
{
  "mcpServers": {
    "renga": {
      "command": "renga",
      "args": ["mcp-server", "--root", "/path/to/project"]
    }
  }
}
```

プロジェクトごとに設定が必要になるが、ファイルベースツールの標準的な解決策。

## 注意点

- `tokio`（非同期ランタイム）の依存が増える → バイナリサイズ増大
- `rmcp` は tokio 必須。stdio transport が `tokio::io::stdin/stdout` に直結しており、他のランタイムへの差し替えは不可
- tokio のバイナリサイズ影響を抑えたいなら feature flags で最小構成にする（`server` + `transport-io` のみ）
- 現在のスキルで機能的には十分。既存スキルは引き続き動くので MCP は追加オプションとして実装すれば既存ユーザーへの影響なし

## 提供すべきツール

| ツール | 対応コマンド |
|---|---|
| `list_issues(status, area, label)` | `renga list` |
| `show_issue(id)` | `renga show` |
| `create_issue(title, area, priority, body)` | `renga create` |
| `done(id)` | `renga done` |
| `pending(id)` | `renga pending` |
| `in_progress(id)` | `renga in-progress` |
| `update_issue(id, ...)` | `renga update` |
| `validate()` | `renga validate` |
