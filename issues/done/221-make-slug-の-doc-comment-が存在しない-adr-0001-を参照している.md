---
schema_version: 1
status: done
priority: medium
area: docs
labels: [found_in_impl]
---

# make_slug の doc comment が存在しない ADR 0001 を参照している

## 問題

`src/issue.rs` の `SLUG_MAX_BYTES` 定数と `make_slug` の doc comment が
「(see ADR 0001)」を2箇所で参照しているが、このリポジトリ（renga/fbim）には
ADR ディレクトリ（`docs/adr/` 等）も ADR 0001 に相当するファイルも存在しない。

```
grep -rn "ADR" --include="*.md" .   # ヒットなし（issues/ 以外）
find . -iname "*adr*"               # ADR ドキュメントは存在しない
```

kiwi モノレポの architect エージェント（`docs/arch/` に ADR を書く）の慣習を
持ち込んでしまった可能性がある。renga/fbim にはそのような ADR 運用は無い。

設計判断の経緯自体は issue #214 の本文（「検討の経緯と決定」節）に詳しく記録されている。

## 対応

- 「(see ADR 0001)」への参照を削除する、または
- 実在する記録（issue #214）を指す記述に変更する（例: 「design rationale: issue #214」）

