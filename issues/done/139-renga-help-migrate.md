---
schema_version: 1
status: done
priority: medium
area: docs
labels: []
---

# renga help migrate が失敗する（インストール済みバイナリが古い）

システムにインストール済みの renga バイナリが古く、migrate サブコマンドを認識しない。
renga help migrate が 'error: unknown command: migrate' で失敗する。
これはリポジトリコードの問題ではなく、インストールバイナリが v0.8.0 相当にアップデートされていないことによる。
install.sh の README への注意書き追加や、CI でのバイナリ検証の検討が必要。
