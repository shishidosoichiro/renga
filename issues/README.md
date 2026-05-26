# Issues

Open issues and decisions.

Status: `open` = needs action / `pending` = blocked or deferred

Closed issues are moved to `done/`.

---

## cli

| # | status | priority | title |
|---|---|---|---|
| [23](23-prev-args-len-2-3.md) | open | medium | 補完の prev を args[len-2] で計算するため引数3トークン以上でフラグ直前値を取得できない |
| [24](24-status-filter-map-ok-area-cli.md) | open | medium | --status フィルタの不正値を filter_map(ok()) で黙殺しエラーも警告も出さない |
| [26](26-show-emit-open-issues-done-iss.md) | open | medium | show の補完が emit_open_issues のみで done issue ID を提示しない |
| [27](27-reopen-emit-done-issues-pendin.md) | open | medium | reopen の補完が emit_done_issues のみで pending issue を提示しない |
| [28](28-fbim-create-area-misc-vs.md) | open | high | fbim create --area のデフォルト値が仕様と不一致（'misc' vs 空文字） |
| [31](31-completions-rs-list-create-mil.md) | open | medium | completions.rs の list / create 補完候補に --milestone が欠落 |
| [35](35-fbim-list-status-unknown.md) | open | low | fbim list --status のヘルプ文字列に unknown が欠落 |

---

## config

| # | status | priority | title |
|---|---|---|---|
| [22](22-fbim-yml-yaml-unwrap-or-defaul.md) | open | medium | .fbim.yml の YAML パースエラーを unwrap_or_default で黙殺しユーザーに無診断のまま default 設定で動き続ける |

---

## core

| # | status | priority | title |
|---|---|---|---|
| [18](18-body-set-frontmatter-field-fro.md) | open | medium | body 中の '---' で set_frontmatter_field が frontmatter 書き換えロジックを再発動しデータ破損する |
| [19](19-reopen-dest-write-path-dest-op.md) | open | medium | reopen が dest への write を path!=dest チェック前に実行し同名 open issue を無警告上書きする |
| [20](20-done-write-remove-file.md) | open | medium | done コマンドの write→remove_file が非アトミックでクラッシュ時に両ディレクトリに矛盾コピーが残る |
| [21](21-create-fs-write-issue.md) | open | medium | create がファイル名衝突チェックなしで fs::write し既存 issue を無警告上書きする |
| [25](25-make-slug-trim-30.md) | open | medium | make_slug が trim 後に 30 バイト切り捨てするため切り捨て位置が '-' のとき末尾ダッシュのファイル名が生成される |
| [34](34-spec.md) | open | medium | タイトル抽出でファイルステムへのフォールバックが未実装（spec と不一致） |

---

## docs

| # | status | priority | title |
|---|---|---|---|
| [12](12-changelog-md-unreleased-keep-a.md) | pending | low | CHANGELOG.md の [Unreleased] セクションと空行が抜けている（Keep a Changelog 形式） |
| [14](14-changelog-keep-a-changelog.md) | pending | medium | CHANGELOG を Keep a Changelog 形式に移行するか検討する |
| [29](29-readme-md-readme-ja-md-fbim-co.md) | open | medium | README.md / README.ja.md のコマンドテーブルに fbim completions が欠落 |
| [30](30-readme-md-readme-ja-md-fbim-li.md) | open | medium | README.md / README.ja.md の fbim list テーブルに --label オプションが欠落 |
| [36](36-readme-fbim-create-area-label-.md) | open | medium | README の fbim create にオプション（--area, --label, --slug 等）が一切記載されていない |

---

## misc

| # | status | priority | title |
|---|---|---|---|
| [38](38-retro-self-improve-area.md) | open | medium | retro: self-improve 経由ルールの違反と新 area 追加 |

---

## test

| # | status | priority | title |
|---|---|---|---|
| [32](32-fbim-init-0.md) | open | medium | fbim init コマンドのテストが存在せずカバレッジ 0% |
| [33](33-readme-rs-62-77.md) | open | low | readme.rs のカバレッジが低い（62.77%） |

---

