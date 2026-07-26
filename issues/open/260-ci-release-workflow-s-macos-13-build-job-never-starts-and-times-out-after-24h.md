---
schema_version: 1
status: open
priority: medium
area: ci
labels: []
---

# ci: release workflow's macos-13 build job never starts and times out after 24h

`.github/workflows/release.yml` の build matrix にある `macos-13`（`x86_64-apple-darwin`）ジョブがランナー待ちのまま起動せず、24時間のタイムアウトで cancel される。

## 実績

| リリース | macos-13 ジョブ |
|---|---|
| v0.16.0 (2026-07-02) | `cancelled` — 24h0m18s 待って起動せず |
| v0.17.0 (2026-07-27) | `queued` のまま（同じ経過をたどる見込み） |

他のジョブは正常:

```
create-release                                     success
publish                                            success   ← crates.io 公開は成功している
build (macos-latest, aarch64-apple-darwin)         success
build (windows-latest, x86_64-pc-windows-msvc)     success
build (ubuntu-latest, x86_64-unknown-linux-gnu)    success
build (macos-13, x86_64-apple-darwin)              cancelled ← これだけ
```

## 影響

- **Intel Mac 向けバイナリ (`x86_64-apple-darwin`) がリリースページに添付されない。** Apple Silicon 版・Linux 版・Windows 版は出ている
- リリースのたびにワークフローが 24 時間 in_progress のまま残る（Actions の実行時間を消費し、リリース完了の判定が曖昧になる）
- crates.io への公開と GitHub リリースノート生成には影響しない

## 原因

GitHub Actions の `macos-13` ランナーラベルは提供終了しており、ジョブが割り当てられない。

## 対応案

1. **`macos-13` を matrix から削除する** — Intel Mac 向けバイナリの配布をやめる。`cargo install renga` は Intel Mac でも動くので、影響は「ビルド済みバイナリが無い」だけ
2. **`macos-latest` 上でクロスコンパイルする** — `rustup target add x86_64-apple-darwin` + `cargo build --target x86_64-apple-darwin`。Apple Silicon ランナーから Intel バイナリを作る。1ジョブで両アーキテクチャを出せる
3. **universal binary にする** — `lipo` で 2 アーキテクチャを結合し、単一の `*-apple-darwin` アセットとして配布

`2` を推奨。Intel Mac のサポートを維持しつつ、ランナー廃止の影響を受けない。

あわせて、build ジョブに `timeout-minutes` を設定して 24 時間ぶら下がらないようにする。

## 関連

- `.github/workflows/release.yml`
- v0.17.0 リリース作業中に発見

