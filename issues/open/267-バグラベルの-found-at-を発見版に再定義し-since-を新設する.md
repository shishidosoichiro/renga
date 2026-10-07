---
schema_version: 1
status: open
priority: low
area: agent
labels: []
---

# バグラベルの found_at を発見版に再定義し since を新設する

## 問題

`.claude/rules/issue-management.md` の `found_at:X.Y.Z` は、「そのバージョンから存在していたバグ」を指す。この定義では、バグを見つけた版と作り込んだ版を区別できない。

## 案

次のように2つのラベルに分ける。

| ラベル | 意味 | 付けるとき |
|---|---|---|
| `found_at:vX.Y.Z` | バグを見つけた版 | 起票時 |
| `since:vX.Y.Z` | バグを作り込んだ版 | 調査で分かったら後から付ける |

分ける利点: リリースノートに「どの版で入った regression か」を書ける。発見時に分からない情報を、推測で埋めずに済む。

## 吟味すべき論点

- Renga の規模で、2つのラベルを使い分ける価値があるか
- 既存の `found_at:` ラベルの意味が変わる。既存の issue を読み替えるか、移行するか
- `/commit` スキルのコミット戦略（`found_in_impl` か `fix:` か）との対応

この変更は `.claude/` を変えるので、retro → self-improve を通す。

