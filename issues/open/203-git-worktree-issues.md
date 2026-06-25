---
schema_version: 1
status: open
priority: medium
area: core
labels: []
---

# git worktree 使用時に issues ディレクトリが分離される

git worktree で linked worktree を作成すると、issues/ が main worktree と分離され、別々の issues ディレクトリを参照してしまう。gitignore されている場合も同様。\n\n検討中の解決策: git rev-parse --git-common-dir を使ってプロジェクトルートを特定する（renga が git コマンドに依存することになるためトレードオフあり）。
