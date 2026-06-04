---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# update --add-label は validate_label を呼ぶが --remove-label は呼ばない

src/commands/update.rs の 51-71 行目で --add-label に対して validate_label を呼んでいるが、--remove-label に対しては呼んでいない。
remove 操作は不正文字を含む label でも実害はないが、一貫性がない。
既存の label が正規化された状態（不正文字を含まない）であれば match しないだけで無害だが、規約の一貫性のために validate_label を add/remove 両方に適用することを検討する。
