---
status: open
priority: high
area: misc
labels: []
---

# Rust への全面書き直し

## 決定済み方針（2026-05-24）

- `lib.rs` を使う。`src/lib.rs` にライブラリクレートを置き、`src/main.rs` からそれを呼ぶ構成にする
- `lib.rs` に `#![deny(missing_docs)]` を置き、公開アイテムの doc コメント漏れをコンパイルエラーにする
- doctest（`# Examples`）は最初は書かない。実装が固まってから追加する
- andrej-karpathy-skills は使わない（Claude Code デフォルト動作と重複するため）
