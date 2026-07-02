---
name: release
description: Release a new version of renga. Use when the user wants to cut a release, publish a new version to crates.io, or tag a new version. The user provides the version number (e.g. "0.4.0") in the conversation.
---

# release skill

renga の新バージョンをリリースする。バージョン番号（例: `0.4.0`）をユーザーから受け取って実行する。

## Steps

1. `renga list` を実行し、open な issue を確認する。リリースに含めるべき未対応 issue がある場合はユーザーに確認する
2. バージョン番号を会話から取得する。指定がなければ、候補を出す前に以下を確認してからユーザーに確認する
   - `git tag --list 'v*' --sort=-version:refname` で最新タグを確認する
   - `cargo search renga --limit 5` または `cargo info renga` で crates.io の最新公開版を確認する
   - `CHANGELOG.md` の最新リリースセクションを確認する
   - `git log <latest-tag>..HEAD --oneline` で前回リリース以降の差分を確認する
   - `feat` / `fix` / breaking change を分類し、semver 候補の根拠を明示する
3. テストがすべて通ることを確認する: `cargo test`
4. `Cargo.toml` の `version` フィールドを新バージョンに更新する
5. `git cliff --tag v<version> -o CHANGELOG.md` で CHANGELOG を生成する（手書き禁止）
6. 変更内容をユーザーに確認する
7. ユーザーの承認を得てからコミット・タグ・プッシュする:
   ```
   git add CHANGELOG.md Cargo.toml
   git commit -m "chore(release): prepare for v<version>"
   git tag v<version>
   git push
   git push origin v<version>
   ```
   - `git push` は origin（GitHub）にプッシュする
   - `git push origin v<version>` で `.github/workflows/release.yml` が起動し、GitHub リリースページにリリースノートが自動生成されるほか、`publish` ジョブが `cargo publish` を自動実行して crates.io に公開する（手動での `cargo publish` は不要。二重実行になる）
8. install スキルを実行してローカルにインストールする
9. 今回のリリースで対応した issue をユーザーに確認し、`renga done <N>` で close する

## Rules

- テストが失敗したらリリースを中止する
- ステップ7の実行前に必ずユーザーの承認を得る
- CHANGELOG は必ず git-cliff で生成する。手書きしない
- バージョンは semver に従う（破壊的変更 → major、機能追加 → minor、バグ修正 → patch）
- 次バージョンは `Cargo.toml` の現在値だけで判断しない。最後に公開されたバージョンから `HEAD` までの user-visible change を根拠にする
