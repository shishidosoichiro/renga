---
status: open
priority: medium
area: config
labels: []
---

# .fbim.yml の YAML パースエラーを unwrap_or_default で黙殺しユーザーに無診断のまま default 設定で動き続ける

- **ファイル**: `src/config.rs:47`
- **再現シナリオ**: `.fbim.yml` にタブ文字等の不正 YAML が含まれると `serde_yaml::from_str` が `Err` を返すが `unwrap_or_default()` で握り潰し、`area_order`/`area_labels`/`issues_dir` がすべてデフォルト値になる。ユーザーにはエラーも警告も表示されない
