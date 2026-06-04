---
status: pending
priority: medium
area: docs
labels: []
---

# CHANGELOG を Keep a Changelog 形式に移行するか検討する

## 背景

現在の CHANGELOG は git-cliff で自動生成しているが、出力形式が独自（絵文字付きカテゴリ）でありユーザー向けとは言いにくい。Keep a Changelog (KAC) 形式への移行を検討したが、いくつかの問題点が明らかになったため保留。

## KAC の利点

- ユーザー視点で変更を記述する標準的な形式
- Added / Changed / Deprecated / Removed / Fixed / Security という明確な区分
- 人間が読みやすく、OSS プロジェクトで広く採用されている

## KAC の欠点・問題点

### 1. KAC は手書きを前提としている

KAC の設計思想は「人間が書く changelog」。公式サイトも「Don't let your friends dump git logs into changelogs」と明言している。コミットログから自動生成することと、KAC の思想は本質的に相容れない。

### 2. コミットタイプと KAC カテゴリのマッピングがロスフルになる

Conventional Commits（feat/fix/refactor...）を KAC（Added/Changed/Removed...）に変換する際、意味が変わる。

- `revert` → Removed とは限らない（バグ修正の revert なら Removed ではない）
- `refactor` → Changed はユーザーへの影響がない変更も含む
- 1つの feat が複数の KAC カテゴリにまたがることもある

### 3. コミット識別子を KAC カテゴリに変える場合、cliff bump が使えなくなる

コミットタイプを added/fixed/changed にすれば git-cliff が直接 KAC 出力できるが、`cliff bump` はConventional Commits の feat/fix/feat! からセマンティックバージョンを計算する。コミット識別子を変えると bump の自動化が失われる。

## 移行する場合の選択肢と考慮点

### A. Conventional Commits のまま、git-cliff の出力フォーマットだけ KAC に寄せる
- マッピングのロスが残る
- `cliff bump` は維持できる
- 生成される文章はユーザー向けとは言えないものになりがち

### B. コミットタイプを KAC カテゴリに変える（added/fixed/changed...）
- git-cliff が直接 KAC カテゴリに振り分けできる
- `cliff bump` が使えなくなる（semver の自動計算が不可能）
- Conventional Commits の標準仕様から外れる

### C. git-cliff でドラフト生成 → リリース前に手編集
- KAC の品質を保てる
- 手間がかかる
- 自動化の恩恵が薄れる

### D. KAC を諦め、現状の git-cliff 出力のまま
- 手間なし
- KAC ではないが changelog としての機能は果たす

## 現状

D（現状維持）で保留。cliff.toml および CHANGELOG.md は変更なし。
