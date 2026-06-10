---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---

# retro: リリース対象範囲を確認せず次バージョンを提案した

リリース相談で、Cargo.toml の現在バージョンだけを見て次バージョンを 0.11.1 と提案した。タグ・CHANGELOG・crates.io 公開済みバージョン・未リリース差分を確認してから semver を判断すべきだった。今後は release スキルのバージョン確認前に、git tag、git log、CHANGELOG、crates.io の状態を確認してから候補を出す。
