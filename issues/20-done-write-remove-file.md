---
status: open
priority: medium
area: core
labels: []
---

# done コマンドの write→remove_file が非アトミックでクラッシュ時に両ディレクトリに矛盾コピーが残る

- **ファイル**: `src/commands/done.rs:27`
- **再現シナリオ**: `fs::write(&dest, &updated)` 成功後に SIGKILL・電源断が起きると、`issues/`（status: open）と `done/`（status: done）に同一ファイルが残る。以後 `fbim list` は open として表示し続け、`fbim done <id>` を再実行すると done 側を黙って上書きする
