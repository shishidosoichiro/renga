---
schema_version: 1
status: open
priority: medium
area: core
labels: []
---

# core: no bulk path repairs the area layout that renga itself created

## 症状

#251 の修正で `canonical_status_dir` は使えない area をフラット扱いにするようになったが、**過去のバージョンが実際に作ってしまったディレクトリを片付ける経路がない**。

- `done`/`pending`/`in-progress`/`reopen`/`update` — 触れた issue だけフラットへ移動する
- `migrate` — warning 付きでスキップ（移動しない）
- `validate --auto-correct` — area をエラー報告するだけで、配置には触れない

実測（`group_by: [area]`、旧版が作った `issues/done/done/1-task.md`）:

```
$ renga validate --auto-correct
error: done/done/1-task.md: area 'done' is not allowed: its slug 'done' collides with a reserved status directory name
$ renga migrate
warning: skipping .../issues/done/done/1-task.md — area 'done' is not allowed: ...
Migrated 0 issue(s).
$ find issues -name '*.md'
issues/done/done/1-task.md     <- 残ったまま
```

結果として、`renga update N --assignee ...` のような無関係な編集を issue ごとに手で叩くまで、renga 自身の `create` が拒否する構成がリポジトリに残り続ける。#251 が掲げた「renga は自分の `create` が拒否するレイアウトを作らない」という原則に対し、既に作ってしまった分の後始末が抜けている。

## もう一点: フォールバックが無警告

`renga done 1` は `issues/2024-q1/open/1-task.md` を `issues/done/1-task.md` へ黙って移動する（実測、stderr に何も出ない）。同じ条件で `migrate` は warning を出すのに、単発コマンドは無言で area のグルーピングを外す。frontmatter の `area:` は保持されるので復旧は可能だが、ユーザーはファイルが area ディレクトリから消えた理由を知る手段がない。

`canonical_status_dir` は `PathBuf` を返す純粋関数なので、集約した結果としてフォールバックの事実を呼び出し側へ伝えられなくなっている点が構造的な原因。

## 対応状況（2026-07-27）

**「フォールバックが無警告」の方は解消済み。** `Context::canonical_dir`（`src/lib.rs`）で `validate_area_for_group_by` が Err を返す area に対し stderr へ warning を出すようにした。

```
$ renga done 1
warning: area '2024 Q1' is not allowed: its slug '2024-q1' starts with an issue ID prefix
warning: filing under done/ without the area grouping
```

`canonical_status_dir`（純粋関数）はそのままに、コマンド層のラッパーである `Context::canonical_dir` に警告を置くことで、呼び出し側 5 箇所（done/pending/in-progress/reopen/update）を一度に賄っている。

**残っているのは一括修復経路の欠落のみ。** これは挙動の設計判断（`migrate` / `validate --auto-correct` を「スキップ」から「フラットへ移動」に変えるか）を伴い、spec.md / spec.ja.md の group_by 段落の書き換えも必要なため、独立した issue として残す。

## 検討する方向

- `migrate` / `validate --auto-correct` でも「使えない area はフラットへ」を適用し、warning 付きで移動する（3 経路の挙動が揃い、一括修復ができる）
- あるいは `canonical_status_dir` の隣に「フォールバックしたか」を返す関数（例: `canonical_status_dir_checked` -> `(PathBuf, Option<FallbackReason>)`）を置き、`done`/`update` 側で warning を出す
- spec.md / spec.ja.md の group_by 段落は「再配置コマンドはフラットに置く」とだけ書いており、`migrate`・`validate --auto-correct` が移動しないこととの差が読み取りにくい。方針決定後に併せて明記する

## 範囲の縮小（2026-07-27）

#243 の ID プレフィックス規則を撤回したため、この issue の対象は**予約ステータス名の area（`area: done` 等）だけ**に縮小した。`issues/2024-q1/` は正当なレイアウトになったので修復対象ではない。

残る「一括修復できないレイアウト」は `issues/done/done/1-task.md` のような、予約名 area の issue を旧版の renga が再配置して作ったものに限られる。該当するのは area にステータス名を使っているプロジェクトのみで、かつ `group_by` 自体が未リリース（v0.16.0 より後の 6fe52d7）のため、実際に踏んでいるユーザーはいない。優先度は下がる。
