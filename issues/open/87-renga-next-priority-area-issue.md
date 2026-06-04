---
status: open
priority: medium
area: cli
labels: []
---

# renga next — priority・area から次に着手すべき issue を返す（AI のトークン節約）

## 疑問

「単純に priority 順で返すだけなら `renga list` の上から取れば同じ」ではないか。`renga next` が `renga list` と差別化できる価値は何か、実装前に明確にする必要がある。

候補：
- 依存関係（blocked_by）を考慮して「今すぐ着手できる issue」だけ返す
- AI のコンテキストに合わせて area でフィルタする
- `--json` 出力 + 1件だけ返すことでトークンを最小化する

## 想定される使い方

```
# AI が作業開始前に次のタスクを確認する
renga next
renga next --area core
```
