---
status: open
priority: medium
area: docs
labels: []
---

# README: Who is it for? の箇条書きを AI エージェントユースケース優先に並べ直す

## 背景

marketing-strategist のレビューで「ターゲットの明確さ」が問題として指摘された。

現在の箇条書き順：
1. Starting a new project and don't want to configure GitHub Issues yet
2. Working offline or on a private machine with no internet access
3. **Using an AI coding tool (like Claude Code)** ← ここに埋まっている
4. Want issues to live in the same git history as the code that fixes them

fbim の最大の差別化は「AI エージェントが直接ファイルとして issue を操作できる」点であり、その訴求が3番目に置かれている。

また「solo developers and small teams」という説明はターゲットとして広すぎるという指摘もあった。「AI エージェントが issue を自律的に操作する」という現代的なユースケースが示されていない。

## 変更方針

- AI エージェントユースケースを箇条書きの1番目に移動する
- 「solo developers and small teams」の表現はそのまま（冒頭を変えるのは #61 の範囲）
- GitHub Issues を批判する表現は避ける。これは単なるトーンの問題ではなく、宍戸さんの方針として「他のツールを非難することはしない」という判断に基づいている。marketing-strategist が「Why not GitHub Issues?」というセクション名を提案したが、宍戸さんはこの書き方に抵抗感を示した。英語圏では比較表現として自然に受け取られることもあるが、それでもこの方針は維持する
- 英語版・日本語版を同期する
