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

## 注意点

- `tokio`（非同期ランタイム）の依存が増える → バイナリサイズ増大
- 現在のスキルで機能的には十分。優先度は低め
- ファイルベースであること自体が renga の価値であり、MCP 対応でその軽量さが薄れる可能性もある

## 提供すべきツール

- `renga_create(title, area, priority)` 
- `renga_list(status, area)`
- `renga_done(id)`
- `renga_show(id)`
- `renga_validate()`
