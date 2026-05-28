use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("issues/done")).unwrap();
    dir
}

fn fbim(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("fbim").unwrap();
    cmd.current_dir(dir.path());
    cmd
}

// ── create ────────────────────────────────────────────────────────────────────

#[test]
fn create_writes_file() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "My Issue", "--area", "core"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1-my-issue.md"));

    let content = fs::read_to_string(dir.path().join("issues/1-my-issue.md")).unwrap();
    assert!(content.contains("schema_version: 1"));
    assert!(content.contains("status: open"));
    assert!(content.contains("area: core"));
    assert!(content.contains("# My Issue"));
}

#[test]
fn create_with_priority_and_body() {
    let dir = setup();
    fbim(&dir)
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

    let content = fs::read_to_string(dir.path().join("issues/1-bug.md")).unwrap();
    assert!(content.contains("priority: high"));
    assert!(content.contains("details here"));
}

#[test]
fn create_with_explicit_slug() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "My Issue", "--slug", "custom-slug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1-custom-slug.md"));
}

#[test]
fn create_sequential_ids() {
    let dir = setup();
    fbim(&dir).args(["create", "First"]).assert().success();
    fbim(&dir)
        .args(["create", "Second"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2-second.md"));
}

#[test]
fn create_without_issues_dir_fails() {
    let dir = TempDir::new().unwrap();
    fbim(&dir)
        .args(["create", "Issue"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("issues directory not found"));
}

// ── list ─────────────────────────────────────────────────────────────────────

#[test]
fn list_shows_open_issues() {
    let dir = setup();
    fbim(&dir).args(["create", "Alpha"]).assert().success();
    fbim(&dir).args(["create", "Beta"]).assert().success();

    fbim(&dir)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alpha"))
        .stdout(predicate::str::contains("Beta"));
}

#[test]
fn list_json_output() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "Issue", "--area", "core"])
        .assert()
        .success();

    fbim(&dir)
        .args(["list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"area\": \"core\""))
        .stdout(predicate::str::contains("\"status\": \"open\""));
}

#[test]
fn list_filters_by_area() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "Core Issue", "--area", "core"])
        .assert()
        .success();
    fbim(&dir)
        .args(["create", "CLI Issue", "--area", "cli"])
        .assert()
        .success();

    fbim(&dir)
        .args(["list", "--area", "core"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Core Issue"))
        .stdout(predicate::str::contains("Core Issue").count(1));
}

#[test]
fn list_filters_by_status() {
    let dir = setup();
    fbim(&dir).args(["create", "Open"]).assert().success();
    fbim(&dir).args(["create", "Will Pend"]).assert().success();
    fbim(&dir).args(["pending", "2"]).assert().success();

    fbim(&dir)
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
    fbim(&dir).args(["create", "Todo"]).assert().success();
    fbim(&dir)
        .args(["done", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("done/1-todo.md"));

    assert!(!dir.path().join("issues/1-todo.md").exists());
    assert!(dir.path().join("issues/done/1-todo.md").exists());

    let content = fs::read_to_string(dir.path().join("issues/done/1-todo.md")).unwrap();
    assert!(content.contains("status: done"));
}

#[test]
fn done_not_found() {
    let dir = setup();
    fbim(&dir)
        .args(["done", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ── pending ───────────────────────────────────────────────────────────────────

#[test]
fn pending_sets_status() {
    let dir = setup();
    fbim(&dir).args(["create", "Work"]).assert().success();
    fbim(&dir).args(["pending", "1"]).assert().success();

    let content = fs::read_to_string(dir.path().join("issues/1-work.md")).unwrap();
    assert!(content.contains("status: pending"));
}

#[test]
fn pending_not_found() {
    let dir = setup();
    fbim(&dir)
        .args(["pending", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ── reopen ────────────────────────────────────────────────────────────────────

#[test]
fn reopen_moves_from_done() {
    let dir = setup();
    fbim(&dir).args(["create", "Old"]).assert().success();
    fbim(&dir).args(["done", "1"]).assert().success();
    fbim(&dir)
        .args(["reopen", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("issues/1-old.md"));

    assert!(dir.path().join("issues/1-old.md").exists());
    assert!(!dir.path().join("issues/done/1-old.md").exists());

    let content = fs::read_to_string(dir.path().join("issues/1-old.md")).unwrap();
    assert!(content.contains("status: open"));
}

#[test]
fn reopen_pending_issue() {
    let dir = setup();
    fbim(&dir).args(["create", "Blocked"]).assert().success();
    fbim(&dir).args(["pending", "1"]).assert().success();
    fbim(&dir).args(["reopen", "1"]).assert().success();

    let content = fs::read_to_string(dir.path().join("issues/1-blocked.md")).unwrap();
    assert!(content.contains("status: open"));
}

// ── show ──────────────────────────────────────────────────────────────────────

#[test]
fn show_prints_content() {
    let dir = setup();
    fbim(&dir).args(["create", "My Issue"]).assert().success();
    fbim(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("# My Issue"))
        .stdout(predicate::str::contains("status: open"));
}

#[test]
fn show_not_found() {
    let dir = setup();
    fbim(&dir)
        .args(["show", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// ── completions ───────────────────────────────────────────────────────────────

#[test]
fn completions_bash() {
    let dir = setup();
    fbim(&dir)
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("__complete"))
        .stdout(predicate::str::contains("fbim"));
}

#[test]
fn completions_zsh() {
    let dir = setup();
    fbim(&dir)
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("__complete"))
        .stdout(predicate::str::contains("fbim"));
}

// ── __complete ────────────────────────────────────────────────────────────────

#[test]
fn complete_lists_subcommands() {
    let dir = setup();
    fbim(&dir)
        .args(["__complete", "fbim", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("done"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn complete_done_shows_open_issues() {
    let dir = setup();
    fbim(&dir).args(["create", "My Task"]).assert().success();

    fbim(&dir)
        .args(["__complete", "fbim", "done", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("1\t"))
        .stdout(predicate::str::contains("My Task"));
}

#[test]
fn complete_reopen_shows_done_issues() {
    let dir = setup();
    fbim(&dir).args(["create", "Old Task"]).assert().success();
    fbim(&dir).args(["done", "1"]).assert().success();

    fbim(&dir)
        .args(["__complete", "fbim", "reopen", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("1\t"))
        .stdout(predicate::str::contains("Old Task"));
}

#[test]
fn complete_list_status_values() {
    let dir = setup();
    fbim(&dir)
        .args(["__complete", "fbim", "list", "--status", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("open"))
        .stdout(predicate::str::contains("pending"))
        .stdout(predicate::str::contains("done"));
}

#[test]
fn reopen_fails_when_open_issue_with_same_name_exists() {
    let dir = setup();
    fbim(&dir).args(["create", "Foo"]).assert().success();
    fbim(&dir).args(["done", "1"]).assert().success();
    // manually place a file with the same name in issues/
    fs::write(
        dir.path().join("issues/1-foo.md"),
        "---\nstatus: open\n---\n\n# Foo\n",
    )
    .unwrap();
    fbim(&dir)
        .args(["reopen", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn create_with_milestone() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "Task", "--milestone", "v1.0"])
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/1-task.md")).unwrap();
    assert!(content.contains("milestone: v1.0"));
}

// ── init ──────────────────────────────────────────────────────────────────────

#[test]
fn init_creates_issues_and_done_dirs() {
    let dir = TempDir::new().unwrap();
    fbim(&dir)
        .args(["init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized"));

    assert!(dir.path().join("issues").is_dir());
    assert!(dir.path().join("issues/done").is_dir());
}

#[test]
fn init_is_idempotent() {
    let dir = TempDir::new().unwrap();
    fbim(&dir).args(["init"]).assert().success();
    fbim(&dir)
        .args(["init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("already initialized"));
    assert!(dir.path().join("issues/done").is_dir());
}

// ── create --body - (stdin) ───────────────────────────────────────────────────

#[test]
fn create_body_from_stdin() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "My Issue", "--body", "-"])
        .write_stdin("body from stdin\n")
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/1-my-issue.md")).unwrap();
    assert!(content.contains("body from stdin"));
}

// ── create --id ───────────────────────────────────────────────────────────────

#[test]
fn create_with_custom_id() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "My Issue", "--id", "99"])
        .assert()
        .success()
        .stdout(predicate::str::contains("99-my-issue.md"));

    assert!(dir.path().join("issues/99-my-issue.md").exists());
}

#[test]
fn create_with_custom_id_collision() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "First", "--id", "5"])
        .assert()
        .success();
    fbim(&dir)
        .args(["create", "Second", "--id", "5", "--slug", "second"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn create_with_id_zero_fails() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "Task", "--id", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("positive integer"));
}

#[test]
fn create_with_id_non_numeric_fails() {
    let dir = setup();
    fbim(&dir)
        .args(["create", "Task", "--id", "abc"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("positive integer"));
}

// ── validate ──────────────────────────────────────────────────────────────────

#[test]
fn validate_clean_issues_exits_ok() {
    let dir = setup();
    fbim(&dir).args(["create", "Task One"]).assert().success();
    fbim(&dir).args(["create", "Task Two"]).assert().success();
    fbim(&dir)
        .args(["validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn validate_detects_unparseable_frontmatter() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    fbim(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("unparseable frontmatter"));
}

#[test]
fn validate_detects_duplicate_ids() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/1-first.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# First\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("issues/1-second.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Second\n",
    )
    .unwrap();
    fbim(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("duplicate ID"));
}

#[test]
fn validate_warns_on_missing_schema_version() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/1-old.md"),
        "---\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Old\n",
    )
    .unwrap();
    fbim(&dir)
        .args(["validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("missing schema_version"));
}

#[test]
fn validate_detects_invalid_status_value() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/1-bad-status.md"),
        "---\nschema_version: 1\nstatus: garbage\npriority: medium\narea: core\nlabels: []\n---\n\n# Bad Status\n",
    )
    .unwrap();
    fbim(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("invalid status value"));
}

#[test]
fn validate_without_issues_dir_fails() {
    let dir = TempDir::new().unwrap();
    fbim(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("issues directory not found"));
}
