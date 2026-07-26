---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# feat: default --dir via defaults.dir config, applied by migrate

`.renga.yml` に `defaults:` ブロック（`dir: Option<bool>`、デフォルト未設定 = 今と同じ flat）を追加し、`renga create` で `--dir` を省略したときのデフォルト値として使う。あわせて `renga migrate` に、`defaults.dir: true` のとき既存の flat issue を dir-based に変換する新ステップを追加する。

## 背景

宍戸さんとの設計会話（2026-07-22）より。常に dir-based で issue を作りたいというワークフロー向けに、毎回 `--dir=true` を指定する手間を無くしたい。

トップレベルに `dir_by_default: bool` を生やす案から、ネストした `defaults: { dir: Option<bool> }` 案に変更した。将来 `assignee`/`priority`/`area` 等の作成時デフォルトが欲しくなったとき、この struct にフィールドを1個足すだけで済み、新しいトップレベルキーの命名や破壊的スキーマ変更を避けられる（Taskwarrior の `default.project`/`default.priority` という `default.*` 名前空間が前例）。

## 設計

- `config.rs`: `Defaults` struct（`dir: Option<bool>`）を追加し、`Config` に `#[serde(default)] pub defaults: Defaults` を追加
- `create.rs`: `let use_dir = args.dir.or(ctx.config.defaults.dir).unwrap_or(false);` に変更
- `migrate.rs`: `defaults.dir == Some(true)` のとき、まだ flat な issue を `update.rs::convert_to_dir` と同じロジックで dir-based に変換する新ステップを追加。衝突（同名ディレクトリが既存等）は group_by の migrate ステップと同じ「warn して skip、全体は止めない」パターンを踏襲する
- **`validate` には追加しない**: area/status と違い、flat か dir-based かは「添付ファイルを持つかどうか」という issue ごとの正当な事情で決まるものであり、`defaults.dir` と異なる状態が「ズレ」とは言えない（意図的に flat のままにしている issue を誤検知することになる）。従って `validate`/`--auto-correct` の対象にはしない
- **一方向のみ実装**: `defaults.dir: true` → flat から dir-based への変換のみ対応する。逆方向（dir-based から flat への一括畳み込み）は `update --dir=false` が添付ファイルありの issue を拒否する仕様上、ノイズの多い操作になりがちなので、需要が出るまで実装しない

## 参考

group_by（issue #229）の migrate ステップと同じ「warn して skip」パターンを再利用する。ただし validate には入れない点が group_by と異なる（上記の判断理由を参照）。
