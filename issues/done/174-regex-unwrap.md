---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# Regex 初期化の unwrap() が非テストコードに残っている

AGENTS.md のエラーハンドリング規約は『unwrap() / expect() はテストコード以外で使わない』としているが、src/issue.rs:15-20 と src/commands/validate.rs:14 に LazyLock<Regex> 初期化の Regex::new(...).unwrap() が残っている。正規表現リテラルなので実害は低いが、規約違反であり、過去の issue #8/#72 が closed になっているにもかかわらず現状コードでは解消されていない。方針を明確にする: (1) 規約どおり Result 返却/コンパイル時生成/OnceLock 初期化エラー処理に置き換える、または (2) static regex literal の unwrap を許容するなら AGENTS.md の規約を self-improve 経由で例外付きに更新する。
