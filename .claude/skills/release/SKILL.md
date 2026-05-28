---
argument-hint: "<version>"
---

# /release skill

renga の新バージョンをリリースする。`$ARGUMENTS` にバージョン番号（例: `0.4.0`）を指定する。

## Steps

1. `renga list` を実行し、open な issue を確認する。リリースに含めるべき未対応 issue がある場合はユーザーに確認する
2. `$ARGUMENTS` からバージョン番号を取得する。指定がなければユーザーに確認する
3. テストがすべて通ることを確認する: `cargo test`
4. `Cargo.toml` の `version` フィールドを新バージョンに更新する
5. `git cliff --tag v<version> -o CHANGELOG.md` で CHANGELOG を生成する（手書き禁止）
6. 変更内容をユーザーに確認する
7. ユーザーの承認を得てからコミット・タグ・プッシュ・publish する:
   ```
   git add CHANGELOG.md Cargo.toml
   git commit -m "chore(release): prepare for v<version>"
   git tag v<version>
   git push
   git push origin v<version>
   cargo publish
   ```
   - `git push` は origin（GitHub）にプッシュする
   - `git push origin v<version>` で `release.yml` が起動し、GitHub リリースページにリリースノートが自動生成される
   - `cargo publish` で crates.io に公開する
8. `/install` スキルを実行してローカルにインストールする
9. 今回のリリースで対応した issue をユーザーに確認し、`renga done <N>` で close する

## Rules

- テストが失敗したらリリースを中止する
- ステップ7の実行前に必ずユーザーの承認を得る
- CHANGELOG は必ず git-cliff で生成する。手書きしない
- バージョンは semver に従う（破壊的変更 → major、機能追加 → minor、バグ修正 → patch）
