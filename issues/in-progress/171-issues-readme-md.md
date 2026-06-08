---
schema_version: 1
status: in-progress
priority: high
area: core
labels: [found_at:0.9.0]
---

# issues/README.md のリンクがステータス別ディレクトリ移行後に壊れている

src/readme.rs:60 と src/readme.rs:89 が issue.path.file_name() だけをリンク先に使っているため、issues/README.md から [87](87-...) のようなリンクが生成される。現在のファイルは issues/open/87-...md や issues/pending/12-...md にあるので、README からクリックすると存在しない issues/87-...md を指す。per-status directories を導入した v0.9.0 以降の回帰として、issues_dir からの相対パス（open/... / pending/...）でリンクを生成し、テストも status subdir 上の path で検証する。
