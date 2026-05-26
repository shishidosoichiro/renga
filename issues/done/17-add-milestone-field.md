---
status: done
priority: medium
area: fbim
labels: []
---

# issue に milestone フィールドを追加して fbim list --milestone でフィルタできるようにする

## 背景

複数のプロジェクト（kiwi 等）でフェーズ・バージョン・スプリントなど「この issue をいつ届けるか」を示すグループ化が必要になった。
現状は area（担当領域）しか分類軸がなく、delivery 単位での絞り込みができない。

## 決定

フィールド名は milestone とする。

iteration も候補だったが、「繰り返す」という意味合いがあり、一回限りのフェーズやバージョンに使うと違和感がある。
milestone は GitHub・GitLab・Linear いずれでもネイティブに採用されており、日付（2026-Q3）・バージョン（v1.0）・フェーズ番号（1-1）・スプリント番号（sprint-3）など開発手法を問わず自然に使える。

## 要件

- issue frontmatter に任意フィールド milestone を追加できる
- fbim list --milestone <value> で絞り込みができる
- fbim create --milestone <value> で作成時に指定できる
- milestone は省略可能（未指定の issue はフィルタ対象外）
