---
argument-hint: "<version>"
---

# /release skill

renga の新バージョンをリリースする。`$ARGUMENTS` にバージョン番号（例: `0.4.0`）を指定する。

## Steps

1. `$ARGUMENTS` からバージョン番号を取得する。指定がなければユーザーに確認する
2. テストがすべて通ることを確認する: `cargo test`
3. `Cargo.toml` の `version` フィールドを新バージョンに更新する
4. `git cliff --tag v<version> -o CHANGELOG.md` で CHANGELOG を生成する（手書き禁止）
5. 変更内容をユーザーに確認する
6. ユーザーの承認を得てからコミット・タグ・プッシュする:
   ```
   git add CHANGELOG.md Cargo.toml
   git commit -m "chore(release): prepare for v<version>"
   git tag v<version>
   git push origin main
   git push origin v<version>
   ```
7. `/install` スキルを実行してローカルにインストールする

## Rules

- テストが失敗したらリリースを中止する
- ステップ6の実行前に必ずユーザーの承認を得る
- CHANGELOG は必ず git-cliff で生成する。手書きしない
- バージョンは semver に従う（破壊的変更 → major、機能追加 → minor、バグ修正 → patch）
