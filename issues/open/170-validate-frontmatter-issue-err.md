---
schema_version: 1
status: open
priority: medium
area: docs
labels: []
---

# spec.md の frontmatter optional 記述と validate の error 判定を整理する

renga validate が frontmatter なし issue を error 扱いする挙動は、検査コマンドとしては妥当。一方で spec.md:50 は 'Frontmatter is optional. When absent, status is unknown...' と書いており、通常の list/read で unknown として読めることと、validate で異常として検出することの区別が曖昧になっている。実装を変えるより、spec.md/spec.ja.md に『通常の読み取りでは unknown として扱うが、validate では missing/unparseable frontmatter を検出対象にする』というようにコマンドごとの意味を明記する。
