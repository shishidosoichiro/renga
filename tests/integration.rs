use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup() -> TempDir {
    let dir = TempDir::new().unwrap();
    for status in ["open", "pending", "in-progress", "done", "unknown"] {
        fs::create_dir_all(dir.path().join(format!("issues/{status}"))).unwrap();
    }
    dir
}

fn renga(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("renga").unwrap();
    cmd.current_dir(dir.path());
    cmd
}

// ── create ────────────────────────────────────────────────────────────────────

#[test]
fn create_writes_file() {
    let dir = setup();
    renga(&dir)
        .args(["create", "My Issue", "--area", "core"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1-my-issue.md"));

    let content = fs::read_to_string(dir.path().join("issues/open/1-my-issue.md")).unwrap();
    assert!(content.contains("schema_version: 1"));
    assert!(content.contains("status: open"));
    assert!(content.contains("area: core"));
    assert!(content.contains("# My Issue"));
}

#[test]
fn create_with_priority_and_body() {
    let dir = setup();
    renga(&dir)
        .args([
            "create",
            "Bug",
            "--priority",
            "high",
            "--body",
            "details here",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/open/1-bug.md")).unwrap();
    assert!(content.contains("priority: high"));
    assert!(content.contains("details here"));
}

#[test]
fn create_with_explicit_slug() {
    let dir = setup();
    renga(&dir)
        .args(["create", "My Issue", "--slug", "custom-slug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1-custom-slug.md"));
}

#[test]
fn create_sequential_ids() {
    let dir = setup();
    renga(&dir).args(["create", "First"]).assert().success();
    renga(&dir)
        .args(["create", "Second"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2-second.md"));
}

#[test]
fn create_without_issues_dir_fails() {
    let dir = TempDir::new().unwrap();
    renga(&dir)
        .args(["create", "Issue"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("issues directory not found"));
}

// ── list ─────────────────────────────────────────────────────────────────────

#[test]
fn list_shows_open_issues() {
    let dir = setup();
    renga(&dir).args(["create", "Alpha"]).assert().success();
    renga(&dir).args(["create", "Beta"]).assert().success();

    renga(&dir)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alpha"))
        .stdout(predicate::str::contains("Beta"));
}

#[test]
fn list_json_output() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Issue", "--area", "core"])
        .assert()
        .success();

    renga(&dir)
        .args(["list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"area\": \"core\""))
        .stdout(predicate::str::contains("\"status\": \"open\""));
}

#[test]
fn list_filters_by_area() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Core Issue", "--area", "core"])
        .assert()
        .success();
    renga(&dir)
        .args(["create", "CLI Issue", "--area", "cli"])
        .assert()
        .success();

    renga(&dir)
        .args(["list", "--area", "core"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Core Issue"))
        .stdout(predicate::str::contains("Core Issue").count(1));
}

#[test]
fn list_filters_by_status() {
    let dir = setup();
    renga(&dir).args(["create", "Open"]).assert().success();
    renga(&dir).args(["create", "Will Pend"]).assert().success();
    renga(&dir).args(["pending", "2"]).assert().success();

    renga(&dir)
        .args(["list", "--status", "pending"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Will Pend"))
        .stdout(predicate::str::contains("Open").not());
}

// ── done ─────────────────────────────────────────────────────────────────────

#[test]
fn done_moves_file_to_done_dir() {
    let dir = setup();
    renga(&dir).args(["create", "Todo"]).assert().success();
    renga(&dir)
        .args(["done", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("done/1-todo.md"));

    assert!(!dir.path().join("issues/open/1-todo.md").exists());
    assert!(dir.path().join("issues/done/1-todo.md").exists());

    let content = fs::read_to_string(dir.path().join("issues/done/1-todo.md")).unwrap();
    assert!(content.contains("status: done"));
}

#[test]
fn done_not_found() {
    let dir = setup();
    renga(&dir)
        .args(["done", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ── pending ───────────────────────────────────────────────────────────────────

#[test]
fn pending_sets_status() {
    let dir = setup();
    renga(&dir).args(["create", "Work"]).assert().success();
    renga(&dir).args(["pending", "1"]).assert().success();

    let content = fs::read_to_string(dir.path().join("issues/pending/1-work.md")).unwrap();
    assert!(content.contains("status: pending"));
}

#[test]
fn pending_not_found() {
    let dir = setup();
    renga(&dir)
        .args(["pending", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ── in-progress ───────────────────────────────────────────────────────────────

#[test]
fn in_progress_sets_status() {
    let dir = setup();
    renga(&dir).args(["create", "Work"]).assert().success();
    renga(&dir).args(["in-progress", "1"]).assert().success();

    let content = fs::read_to_string(dir.path().join("issues/in-progress/1-work.md")).unwrap();
    assert!(content.contains("status: in-progress"));
}

#[test]
fn in_progress_not_found() {
    let dir = setup();
    renga(&dir)
        .args(["in-progress", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ── reopen ────────────────────────────────────────────────────────────────────

#[test]
fn reopen_moves_from_done() {
    let dir = setup();
    renga(&dir).args(["create", "Old"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();
    renga(&dir)
        .args(["reopen", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("issues/open/1-old.md"));

    assert!(dir.path().join("issues/open/1-old.md").exists());
    assert!(!dir.path().join("issues/done/1-old.md").exists());

    let content = fs::read_to_string(dir.path().join("issues/open/1-old.md")).unwrap();
    assert!(content.contains("status: open"));
}

#[test]
fn reopen_pending_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Blocked"]).assert().success();
    renga(&dir).args(["pending", "1"]).assert().success();
    renga(&dir).args(["reopen", "1"]).assert().success();

    let content = fs::read_to_string(dir.path().join("issues/open/1-blocked.md")).unwrap();
    assert!(content.contains("status: open"));
}

#[test]
fn reopen_in_progress_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Active"]).assert().success();
    renga(&dir).args(["in-progress", "1"]).assert().success();
    renga(&dir).args(["reopen", "1"]).assert().success();

    let content = fs::read_to_string(dir.path().join("issues/open/1-active.md")).unwrap();
    assert!(content.contains("status: open"));
}

// ── show ──────────────────────────────────────────────────────────────────────

#[test]
fn show_prints_content() {
    let dir = setup();
    renga(&dir).args(["create", "My Issue"]).assert().success();
    renga(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("# My Issue"))
        .stdout(predicate::str::contains("status: open"));
}

#[test]
fn show_json_output() {
    let dir = setup();
    renga(&dir)
        .args(["create", "My Issue", "--area", "core", "--label", "bug"])
        .assert()
        .success();

    renga(&dir)
        .args(["show", "1", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"id\": \"1\""))
        .stdout(predicate::str::contains("\"area\": \"core\""))
        .stdout(predicate::str::contains("\"status\": \"open\""))
        .stdout(predicate::str::contains("\"bug\""))
        .stdout(predicate::str::contains("\"title\": \"My Issue\""));
}

#[test]
fn show_json_done_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Done Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["show", "1", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"done\""));
}

#[test]
fn show_not_found() {
    let dir = setup();
    renga(&dir)
        .args(["show", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ── completions ───────────────────────────────────────────────────────────────

#[test]
fn completions_bash() {
    let dir = setup();
    renga(&dir)
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("__complete"))
        .stdout(predicate::str::contains("renga"));
}

#[test]
fn completions_zsh() {
    let dir = setup();
    renga(&dir)
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("__complete"))
        .stdout(predicate::str::contains("renga"));
}

// ── __complete ────────────────────────────────────────────────────────────────

#[test]
fn complete_lists_subcommands() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("done"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn complete_done_shows_open_issues() {
    let dir = setup();
    renga(&dir).args(["create", "My Task"]).assert().success();

    renga(&dir)
        .args(["__complete", "renga", "done", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("1\t"))
        .stdout(predicate::str::contains("My Task"));
}

#[test]
fn complete_reopen_shows_done_issues() {
    let dir = setup();
    renga(&dir).args(["create", "Old Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["__complete", "renga", "reopen", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("1\t"))
        .stdout(predicate::str::contains("Old Task"));
}

#[test]
fn complete_list_status_values() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "list", "--status", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("open"))
        .stdout(predicate::str::contains("pending"))
        .stdout(predicate::str::contains("done"));
}

#[test]
fn reopen_fails_when_open_issue_with_same_name_exists() {
    let dir = setup();
    renga(&dir).args(["create", "Foo"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();
    // manually place a file with the same name in issues/
    fs::write(
        dir.path().join("issues/open/1-foo.md"),
        "---\nstatus: open\n---\n\n# Foo\n",
    )
    .unwrap();
    renga(&dir)
        .args(["reopen", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn create_with_labels() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--label", "bug", "--label", "urgent"])
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("labels: [bug, urgent]"));
}

#[test]
fn create_label_with_comma_fails() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--label", "bug, urgent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid character"));
}

#[test]
fn create_with_milestone() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--milestone", "v1.0"])
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("milestone: v1.0"));
}

// ── info ──────────────────────────────────────────────────────────────────────

#[test]
fn info_shows_paths_and_defaults() {
    let dir = setup();
    let output = renga(&dir).args(["info"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("project root:"));
    assert!(stdout.contains("issues dir:"));
    assert!(stdout.contains("not found — using defaults"));
    assert!(stdout.contains("issues_dir"));
    assert!(stdout.contains("area_order"));
    assert!(stdout.contains("area_labels"));
}

#[test]
fn info_shows_config_when_present() {
    let dir = setup();
    fs::write(
        dir.path().join(".renga.yml"),
        "area_order: [core, cli]\narea_labels:\n  core: \"Core\"\n",
    )
    .unwrap();
    let output = renga(&dir).args(["info"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(!stdout.contains("not found"));
    assert!(stdout.contains("core, cli"));
    assert!(stdout.contains("core → \"Core\""));
}

// ── init ──────────────────────────────────────────────────────────────────────

#[test]
fn init_creates_issues_and_done_dirs() {
    let dir = TempDir::new().unwrap();
    renga(&dir)
        .args(["init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized"));

    assert!(dir.path().join("issues").is_dir());
    assert!(dir.path().join("issues/open").is_dir());
    assert!(dir.path().join("issues/done").is_dir());
    assert!(dir.path().join("issues/pending").is_dir());
    assert!(dir.path().join("issues/in-progress").is_dir());
    assert!(dir.path().join("issues/unknown").is_dir());
}

#[test]
fn init_is_idempotent() {
    let dir = TempDir::new().unwrap();
    renga(&dir).args(["init"]).assert().success();
    renga(&dir)
        .args(["init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("already initialized"));
    assert!(dir.path().join("issues/open").is_dir());
}

// ── create --body - (stdin) ───────────────────────────────────────────────────

#[test]
fn create_body_from_stdin() {
    let dir = setup();
    renga(&dir)
        .args(["create", "My Issue", "--body", "-"])
        .write_stdin("body from stdin\n")
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/open/1-my-issue.md")).unwrap();
    assert!(content.contains("body from stdin"));
}

// ── create --id ───────────────────────────────────────────────────────────────

#[test]
fn create_with_custom_id() {
    let dir = setup();
    renga(&dir)
        .args(["create", "My Issue", "--id", "99"])
        .assert()
        .success()
        .stdout(predicate::str::contains("99-my-issue.md"));

    assert!(dir.path().join("issues/open/99-my-issue.md").exists());
}

#[test]
fn create_with_custom_id_collision() {
    let dir = setup();
    renga(&dir)
        .args(["create", "First", "--id", "5"])
        .assert()
        .success();
    renga(&dir)
        .args(["create", "Second", "--id", "5", "--slug", "second"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn create_with_id_zero_fails() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--id", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("positive integer"));
}

#[test]
fn create_with_id_non_numeric_fails() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--id", "abc"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("positive integer"));
}

// ── migrate ───────────────────────────────────────────────────────────────────

#[test]
fn migrate_moves_flat_files_to_status_dirs() {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("issues")).unwrap();
    fs::write(
        dir.path().join("issues/1-open-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Open Task\n",
    ).unwrap();
    fs::write(
        dir.path().join("issues/2-pending-task.md"),
        "---\nschema_version: 1\nstatus: pending\npriority: medium\narea: core\nlabels: []\n---\n\n# Pending Task\n",
    ).unwrap();
    fs::write(
        dir.path().join("issues/3-no-frontmatter.md"),
        "# No Frontmatter\n",
    )
    .unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrated 3 issue(s)."));

    assert!(dir.path().join("issues/open/1-open-task.md").exists());
    assert!(dir.path().join("issues/pending/2-pending-task.md").exists());
    assert!(dir
        .path()
        .join("issues/unknown/3-no-frontmatter.md")
        .exists());
    assert!(!dir.path().join("issues/1-open-task.md").exists());
}

#[test]
fn migrate_nothing_to_migrate() {
    let dir = setup();
    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Nothing to migrate."));
}

#[test]
fn migrate_skips_collision() {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("issues/open")).unwrap();
    // flat file
    fs::write(
        dir.path().join("issues/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    // already in open/
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: high\narea: core\nlabels: []\n---\n\n# Task (existing)\n",
    )
    .unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stderr(predicate::str::contains("skipping"));

    // existing file in open/ must be preserved
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("Task (existing)"));
}

#[test]
fn update_status_moves_to_in_progress() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--status", "in-progress"])
        .assert()
        .success();
    assert!(dir.path().join("issues/in-progress/1-task.md").exists());
    assert!(!dir.path().join("issues/open/1-task.md").exists());
}

// ── validate ──────────────────────────────────────────────────────────────────

#[test]
fn validate_clean_issues_exits_ok() {
    let dir = setup();
    renga(&dir).args(["create", "Task One"]).assert().success();
    renga(&dir).args(["create", "Task Two"]).assert().success();
    renga(&dir)
        .args(["validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn validate_detects_unparseable_frontmatter() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("unparseable frontmatter"));
}

#[test]
fn validate_detects_duplicate_ids() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-first.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# First\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("issues/open/1-second.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Second\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("duplicate ID"));
}

#[test]
fn validate_warns_on_missing_schema_version() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-old.md"),
        "---\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Old\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("missing schema_version"));
}

#[test]
fn validate_detects_invalid_status_value() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-bad-status.md"),
        "---\nschema_version: 1\nstatus: garbage\npriority: medium\narea: core\nlabels: []\n---\n\n# Bad Status\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("invalid status value"));
}

#[test]
fn validate_without_issues_dir_fails() {
    let dir = TempDir::new().unwrap();
    renga(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("issues directory not found"));
}

// ── update ────────────────────────────────────────────────────────────────────

#[test]
fn update_priority() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--priority", "high"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("priority: high"));
}

#[test]
fn update_area() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--area", "core"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("area: core"));
}

#[test]
fn update_labels() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--label", "bug", "--label", "urgent"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("bug"));
    assert!(content.contains("urgent"));
}

#[test]
fn update_body() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--body", "new body text"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("new body text"));
}

#[test]
fn update_title_positional() {
    let dir = setup();
    renga(&dir).args(["create", "Old Title"]).assert().success();
    renga(&dir)
        .args(["update", "1", "New Title"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-old-title.md")).unwrap();
    assert!(content.contains("# New Title"));
    assert!(!content.contains("# Old Title"));
}

#[test]
fn update_body_preserves_existing_title() {
    let dir = setup();
    renga(&dir).args(["create", "My Issue"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--body", "description without heading"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-my-issue.md")).unwrap();
    assert!(content.contains("# My Issue"));
    assert!(content.contains("description without heading"));
}

#[test]
fn update_title_and_body_together() {
    let dir = setup();
    renga(&dir).args(["create", "Original"]).assert().success();
    renga(&dir)
        .args(["update", "1", "Updated Title", "--body", "new body text"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-original.md")).unwrap();
    assert!(content.contains("# Updated Title"));
    assert!(content.contains("new body text"));
}

#[test]
fn update_body_with_heading_uses_provided_heading() {
    let dir = setup();
    renga(&dir).args(["create", "My Issue"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--body", "# New Heading\n\nbody text"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-my-issue.md")).unwrap();
    assert!(content.contains("# New Heading"));
    assert!(!content.contains("# My Issue"));
}

#[test]
fn update_add_label() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--label", "bug"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--add-label", "urgent"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("bug"));
    assert!(content.contains("urgent"));
}

#[test]
fn update_remove_label() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--label", "bug", "--label", "urgent"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--remove-label", "urgent"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("bug"));
    assert!(!content.contains("urgent"));
}

#[test]
fn update_add_label_no_duplicates() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--label", "bug"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--add-label", "bug"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert_eq!(content.matches("bug").count(), 1);
}

#[test]
fn update_status_moves_file() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    assert!(dir.path().join("issues/open/1-task.md").exists());

    renga(&dir)
        .args(["update", "1", "--status", "pending"])
        .assert()
        .success();

    assert!(dir.path().join("issues/pending/1-task.md").exists());
    assert!(!dir.path().join("issues/open/1-task.md").exists());
    let content = fs::read_to_string(dir.path().join("issues/pending/1-task.md")).unwrap();
    assert!(content.contains("status: pending"));
}

#[test]
fn update_not_found() {
    let dir = setup();
    renga(&dir)
        .args(["update", "99", "--priority", "high"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("99"));
}

#[test]
fn update_body_preserves_frontmatter() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--area", "core", "--priority", "high"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--body", "updated body"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("area: core"));
    assert!(content.contains("priority: high"));
    assert!(content.contains("updated body"));
    assert!(
        !content.contains("area: core---"),
        "frontmatter must not be corrupted"
    );
}

#[test]
fn update_body_from_stdin() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--body", "-"])
        .write_stdin("stdin body\n")
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("stdin body"));
}

#[test]
fn edit_not_found_fails() {
    let dir = setup();
    std::env::set_var("EDITOR", "true");
    renga(&dir)
        .args(["edit", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("99"));
}

#[test]
fn edit_opens_editor() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .env("EDITOR", "true")
        .args(["edit", "1"])
        .assert()
        .success();
}
