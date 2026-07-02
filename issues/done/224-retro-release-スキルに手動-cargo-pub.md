---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---

# retro: /release スキルに手動 cargo publish の古い記載が残っている

## 経緯

v0.16.0 リリース時、`.claude/skills/release/SKILL.md` の step 7 に
「`cargo publish` を手動実行する」という手順が書かれたままだった。しかし
`1dc6f89 ci: publish to crates.io on release tag`（v0.15.0 リリース時に
導入）で `.github/workflows/release.yml` に `publish` ジョブが追加され、
`git push origin v<version>` でタグを push すると GitHub Actions が
自動で `cargo publish` を実行するようになっている。

スキルの手順書が CI 自動化の変更に追従しておらず、古い手動手順のまま
残っていた。実際には手動 `cargo publish` は不要（二重実行になる恐れもある）。

## 改善内容

`.claude/skills/release/SKILL.md` の step 7 を「タグ push で GitHub Actions
が自動的に publish する」旨に更新し、手動 `cargo publish` の記載を削除する。

