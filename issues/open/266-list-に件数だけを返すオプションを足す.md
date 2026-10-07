---
type: Issue
schema_version: 1
status: open
priority: low
area: cli
labels: []
---

# list に件数だけを返すオプションを足す

## 問題

「条件に合う未完了の issue が0件か」をゲートに使う場面がある。たとえば、リリース前に、その版のラベルを持つ未完了の issue が残っていないかを確かめる。この判定に、件数だけを返す手段が無い。今は jq で判定するしかない。

```sh
renga list --label "target_version:vX.Y.Z" --json | jq '[.[] | select(.status != "done")] | length == 0'
```

## 期待する動作（案）

- `renga list ... --count` で件数だけを出す
- 0件かどうかを exit code で返す案もある。ただし grep の慣習（一致なしで 1）に倣うかは要検討

優先度は低め。jq で補えるため。

