---
status: open
priority: low
area: cli
labels: []
---

# fbim list --status のヘルプ文字列に unknown が欠落

## 問題

src/cli.rs 117行目の ListArgs::status フィールドの doc コメントに "unknown" が含まれていない。

現在: `/// Filter by status. Comma-separated: `open`, `pending`, `done`.`
正しくは: `/// Filter by status. Comma-separated: `open`, `pending`, `done`, `unknown`.`

そのため `fbim list --help` の --status ヘルプ出力が:
`Filter by status. Comma-separated: `open`, `pending`, `done``
となり unknown が欠落している。

spec.md 49-53行目では unknown は有効な status 値として明記されている。

## 関連箇所

- src/cli.rs:117 ListArgs::status の doc コメント
- spec.md:49-53 status の値一覧
