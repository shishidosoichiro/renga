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

#[test]
fn done_rejects_normal_done_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["done", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn done_operates_on_misplaced_active_issue_with_warning() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-misplaced.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Misplaced\n",
    )
    .unwrap();

    renga(&dir)
        .args(["done", "1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("stored in done/"))
        .stderr(predicate::str::contains("renga validate 1 --auto-correct"));

    let content = fs::read_to_string(dir.path().join("issues/done/1-misplaced.md")).unwrap();
    assert!(content.contains("status: done"));
}

#[test]
fn done_multiple_ids() {
    let dir = setup();
    renga(&dir).args(["create", "First"]).assert().success();
    renga(&dir).args(["create", "Second"]).assert().success();
    renga(&dir).args(["create", "Third"]).assert().success();

    renga(&dir).args(["done", "1", "2", "3"]).assert().success();

    assert!(dir.path().join("issues/done/1-first.md").exists());
    assert!(dir.path().join("issues/done/2-second.md").exists());
    assert!(dir.path().join("issues/done/3-third.md").exists());
}

#[test]
fn done_partial_failure() {
    let dir = setup();
    renga(&dir).args(["create", "Exists"]).assert().success();

    renga(&dir)
        .args(["done", "1", "99"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("1-exists.md"))
        .stderr(predicate::str::contains("99"))
        .stderr(predicate::str::contains("not found"));

    assert!(dir.path().join("issues/done/1-exists.md").exists());
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

#[test]
fn pending_rejects_normal_done_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["pending", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn pending_operates_on_misplaced_active_issue_with_warning() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-misplaced.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Misplaced\n",
    )
    .unwrap();

    renga(&dir)
        .args(["pending", "1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("stored in done/"));

    assert!(dir.path().join("issues/pending/1-misplaced.md").exists());
}

#[test]
fn pending_does_not_operate_on_done_issue_with_missing_status() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-missing-status.md"),
        "---\nschema_version: 1\npriority: medium\narea: core\nlabels: []\n---\n\n# Missing status\n",
    )
    .unwrap();

    renga(&dir)
        .args(["pending", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));

    assert!(dir.path().join("issues/done/1-missing-status.md").exists());
    assert!(!dir
        .path()
        .join("issues/pending/1-missing-status.md")
        .exists());
}

#[test]
fn pending_multiple_ids() {
    let dir = setup();
    renga(&dir).args(["create", "Alpha"]).assert().success();
    renga(&dir).args(["create", "Beta"]).assert().success();

    renga(&dir).args(["pending", "1", "2"]).assert().success();

    assert!(dir.path().join("issues/pending/1-alpha.md").exists());
    assert!(dir.path().join("issues/pending/2-beta.md").exists());
}

#[test]
fn pending_partial_failure() {
    let dir = setup();
    renga(&dir).args(["create", "Exists"]).assert().success();

    renga(&dir)
        .args(["pending", "1", "99"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("1-exists.md"))
        .stderr(predicate::str::contains("99"))
        .stderr(predicate::str::contains("not found"));

    assert!(dir.path().join("issues/pending/1-exists.md").exists());
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

#[test]
fn in_progress_rejects_normal_done_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["in-progress", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn in_progress_operates_on_misplaced_active_issue_with_warning() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-misplaced.md"),
        "---\nschema_version: 1\nstatus: pending\npriority: medium\narea: core\nlabels: []\n---\n\n# Misplaced\n",
    )
    .unwrap();

    renga(&dir)
        .args(["in-progress", "1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("stored in done/"));

    assert!(dir
        .path()
        .join("issues/in-progress/1-misplaced.md")
        .exists());
}

#[test]
fn in_progress_multiple_ids() {
    let dir = setup();
    renga(&dir).args(["create", "Task A"]).assert().success();
    renga(&dir).args(["create", "Task B"]).assert().success();

    renga(&dir)
        .args(["in-progress", "1", "2"])
        .assert()
        .success();

    assert!(dir.path().join("issues/in-progress/1-task-a.md").exists());
    assert!(dir.path().join("issues/in-progress/2-task-b.md").exists());
}

#[test]
fn in_progress_partial_failure() {
    let dir = setup();
    renga(&dir).args(["create", "Exists"]).assert().success();

    renga(&dir)
        .args(["in-progress", "1", "99"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("1-exists.md"))
        .stderr(predicate::str::contains("99"))
        .stderr(predicate::str::contains("not found"));

    assert!(dir.path().join("issues/in-progress/1-exists.md").exists());
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

#[test]
fn reopen_multiple_ids() {
    let dir = setup();
    renga(&dir).args(["create", "First"]).assert().success();
    renga(&dir).args(["create", "Second"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();
    renga(&dir).args(["done", "2"]).assert().success();

    renga(&dir).args(["reopen", "1", "2"]).assert().success();

    assert!(dir.path().join("issues/open/1-first.md").exists());
    assert!(dir.path().join("issues/open/2-second.md").exists());
}

#[test]
fn reopen_partial_failure() {
    let dir = setup();
    renga(&dir).args(["create", "Exists"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["reopen", "1", "99"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("1-exists.md"))
        .stderr(predicate::str::contains("99"))
        .stderr(predicate::str::contains("not found"));

    assert!(dir.path().join("issues/open/1-exists.md").exists());
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

#[test]
fn completions_fish() {
    let dir = setup();
    renga(&dir)
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("# fish completion for renga"))
        .stdout(predicate::str::contains("complete -c renga -f"))
        .stdout(predicate::str::contains("renga __complete $tokens"));
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
fn complete_lists_global_flags() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "-"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--version\tPrint version"))
        .stdout(predicate::str::contains("--help\tPrint help"));
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
        .stdout(predicate::str::contains("open\tActive issue"))
        .stdout(predicate::str::contains("pending\tBlocked or deferred"))
        .stdout(predicate::str::contains("done\tThe issue is complete"));
}

#[test]
fn complete_list_flags() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "list", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "--status\tFilter by status. Comma-separated:",
        ))
        .stdout(predicate::str::contains("--area\tFilter by area"))
        .stdout(predicate::str::contains("--json\tOutput as JSON"));
}

#[test]
fn complete_completions_shell_names_with_descriptions() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "completions", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("bash\tBash"))
        .stdout(predicate::str::contains("zsh\tZsh"))
        .stdout(predicate::str::contains("fish\tFish"));
}

#[test]
fn complete_create_flags() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "create", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "--id\tIssue ID to use instead of auto-incrementing",
        ))
        .stdout(predicate::str::contains("--priority\tPriority level"))
        .stdout(predicate::str::contains("--label\tLabels to attach"))
        .stdout(predicate::str::contains(
            "--json\tRead issue fields as JSON from stdin",
        ));
}

#[test]
fn complete_create_priority_values_with_descriptions() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "create", "--priority", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("high\tHigh priority"))
        .stdout(predicate::str::contains("medium\tMedium priority"))
        .stdout(predicate::str::contains("low\tLow priority"));
}

#[test]
fn complete_update_shows_open_issues_and_flags() {
    let dir = setup();
    renga(&dir).args(["create", "My Task"]).assert().success();

    renga(&dir)
        .args(["__complete", "renga", "update", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("1\t"))
        .stdout(predicate::str::contains("My Task"))
        .stdout(predicate::str::contains("--priority\tNew priority level"))
        .stdout(predicate::str::contains("--status\tNew status"))
        .stdout(predicate::str::contains(
            "--json\tRead fields to update as JSON from stdin",
        ));
}

#[test]
fn complete_update_priority_values_with_descriptions() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "update", "--priority", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("high\tHigh priority"))
        .stdout(predicate::str::contains("medium\tMedium priority"))
        .stdout(predicate::str::contains("low\tLow priority"));
}

#[test]
fn complete_update_status_values_with_descriptions() {
    let dir = setup();
    renga(&dir)
        .args(["__complete", "renga", "update", "--status", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("open\tActive issue"))
        .stdout(predicate::str::contains("pending\tBlocked or deferred"))
        .stdout(predicate::str::contains(
            "in-progress\tActively being worked on",
        ));
}

#[test]
fn complete_validate_shows_all_issues_and_flags() {
    let dir = setup();
    renga(&dir).args(["create", "Open Task"]).assert().success();
    renga(&dir).args(["create", "Done Task"]).assert().success();
    renga(&dir).args(["done", "2"]).assert().success();

    renga(&dir)
        .args(["__complete", "renga", "validate", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("1\t"))
        .stdout(predicate::str::contains("Open Task"))
        .stdout(predicate::str::contains("2\t"))
        .stdout(predicate::str::contains("Done Task"))
        .stdout(predicate::str::contains(
            "--auto-correct\tMove files to the status directory declared in frontmatter",
        ));
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

#[test]
fn create_from_json_stdin() {
    let dir = setup();
    renga(&dir)
        .args(["create", "--json"])
        .write_stdin(
            r#"{"title":"JSON Issue","slug":"json-issue","priority":"high","area":"cli","body":"body from json","milestone":"v1","labels":["bug","urgent"]}"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("1-json-issue.md"));

    let content = fs::read_to_string(dir.path().join("issues/open/1-json-issue.md")).unwrap();
    assert!(content.contains("priority: high"));
    assert!(content.contains("area: cli"));
    assert!(content.contains("milestone: v1"));
    assert!(content.contains("labels: [bug, urgent]"));
    assert!(content.contains("# JSON Issue"));
    assert!(content.contains("body from json"));
}

#[test]
fn create_from_json_requires_title() {
    let dir = setup();
    renga(&dir)
        .args(["create", "--json"])
        .write_stdin(r#"{"area":"cli"}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("title"));
}

#[test]
fn create_json_rejects_cli_fields() {
    let dir = setup();
    renga(&dir)
        .args(["create", "--json", "--area", "cli"])
        .write_stdin(r#"{"title":"Task"}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("--json cannot be combined"));

    renga(&dir)
        .args(["create", "--json", "--priority", "medium"])
        .write_stdin(r#"{"title":"Task"}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("--json cannot be combined"));
}

#[test]
fn create_json_rejects_unknown_fields() {
    let dir = setup();
    renga(&dir)
        .args(["create", "--json"])
        .write_stdin(r#"{"title":"Task","label":["bug"]}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown field"));
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
fn migrate_ignores_digit_led_files_without_a_hyphen() {
    // Regression guard: migrate's flat-file filter used to accept any
    // top-level .md file starting with an ASCII digit, without requiring
    // the `N-slug` hyphen separator that the rest of renga's ID grammar
    // (id_prefix) requires. Two such malformed files both migrating
    // successfully but both producing an empty extract_id() collided in
    // migrate's ID-keyed dedup set and undercounted "Migrated N issue(s).".
    // The fix is to only ever treat real `N-slug.md` files as candidates,
    // so malformed files are left untouched (they were never recognized as
    // issues by the rest of the toolchain anyway).
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("issues")).unwrap();
    fs::write(dir.path().join("issues/5foo.md"), "# No hyphen\n").unwrap();
    fs::write(dir.path().join("issues/7bar.md"), "# No hyphen 2\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Nothing to migrate."));

    assert!(dir.path().join("issues/5foo.md").exists());
    assert!(dir.path().join("issues/7bar.md").exists());
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
fn validate_id_only_checks_selected_issue() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-good.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Good\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("issues/open/2-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn validate_selected_id_detects_duplicate_candidates() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-first.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# First\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("issues/done/1-second.md"),
        "---\nschema_version: 1\nstatus: done\npriority: medium\narea: core\nlabels: []\n---\n\n# Second\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate", "1"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("duplicate ID"));
}

#[test]
fn validate_selected_id_detects_duplicate_even_when_frontmatter_is_bad() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-first.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# First\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("issues/done/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate", "1"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("unparseable frontmatter"))
        .stdout(predicate::str::contains("duplicate ID"));
}

#[test]
fn validate_selected_missing_id_fails() {
    let dir = setup();
    renga(&dir)
        .args(["validate", "99"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("issue not found"));
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
fn validate_detects_missing_status() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-missing-status.md"),
        "---\nschema_version: 1\npriority: medium\narea: core\nlabels: []\n---\n\n# Missing Status\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate", "1", "--auto-correct"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("missing status"));

    assert!(dir.path().join("issues/done/1-missing-status.md").exists());
    assert!(!dir.path().join("issues/open/1-missing-status.md").exists());
}

#[test]
fn validate_detects_status_directory_mismatch() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-mismatch.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Mismatch\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("status directory mismatch"));
}

#[test]
fn validate_auto_correct_moves_to_frontmatter_status_directory() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-mismatch.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Mismatch\n",
    )
    .unwrap();
    renga(&dir)
        .args(["validate", "1", "--auto-correct"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "corrected: done/1-mismatch.md -> open/1-mismatch.md",
        ));

    assert!(dir.path().join("issues/open/1-mismatch.md").exists());
    assert!(!dir.path().join("issues/done/1-mismatch.md").exists());
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
fn update_milestone_adds_missing_field() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--milestone", "v1"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("milestone: v1"));
}

#[test]
fn update_rejects_unparseable_frontmatter() {
    let dir = setup();
    let path = dir.path().join("issues/open/1-bad.md");
    fs::write(&path, "---\nnot: valid: yaml: [\n---\n\n# Bad\n").unwrap();
    renga(&dir)
        .args(["update", "1", "--milestone", "v1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid frontmatter"));
    let content = fs::read_to_string(path).unwrap();
    assert!(!content.contains("milestone: v1"));
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
fn update_status_moves_done_issue_to_pending() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();
    assert!(dir.path().join("issues/done/1-task.md").exists());

    renga(&dir)
        .args(["update", "1", "--status", "pending"])
        .assert()
        .success();

    assert!(!dir.path().join("issues/done/1-task.md").exists());
    assert!(dir.path().join("issues/pending/1-task.md").exists());
    let content = fs::read_to_string(dir.path().join("issues/pending/1-task.md")).unwrap();
    assert!(content.contains("status: pending"));
}

#[test]
fn update_operates_on_misplaced_active_issue_with_warning() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-misplaced.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Misplaced\n",
    )
    .unwrap();

    renga(&dir)
        .args(["update", "1", "--assignee", "alice"])
        .assert()
        .success()
        .stderr(predicate::str::contains("stored in done/"));

    // update relocates to the canonical directory for the frontmatter status
    // as a side effect (self-healing), so the file no longer sits under done/.
    assert!(!dir.path().join("issues/done/1-misplaced.md").exists());
    let content = fs::read_to_string(dir.path().join("issues/open/1-misplaced.md")).unwrap();
    assert!(content.contains("assignee: alice"));
}

#[test]
fn update_can_edit_normal_done_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["update", "1", "--assignee", "alice"])
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/done/1-task.md")).unwrap();
    assert!(content.contains("assignee: alice"));
    assert!(content.contains("status: done"));
}

#[test]
fn update_can_edit_dir_based_done_issue() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .args(["update", "1", "--assignee", "alice"])
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join("issues/done/1-task/README.md")).unwrap();
    assert!(content.contains("assignee: alice"));
    assert!(content.contains("status: done"));
}

#[test]
fn update_rejects_done_issue_missing_status_field() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-task.md"),
        "---\nschema_version: 1\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();

    renga(&dir)
        .args(["update", "1", "--assignee", "alice"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn edit_can_edit_normal_done_issue() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .env("EDITOR", "true")
        .args(["edit", "1"])
        .assert()
        .success();
}

#[test]
fn edit_can_edit_dir_based_done_issue() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    renga(&dir).args(["done", "1"]).assert().success();

    renga(&dir)
        .env("EDITOR", "true")
        .args(["edit", "1"])
        .assert()
        .success();
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
fn update_from_json_stdin() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--label", "old", "--milestone", "v1"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--json"])
        .write_stdin(
            r#"{"title":"JSON Title","priority":"low","area":"core","status":"pending","milestone":"v2","labels":["new"],"add_labels":["urgent"],"body":"json body"}"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("issues/pending/1-task.md"));

    let content = fs::read_to_string(dir.path().join("issues/pending/1-task.md")).unwrap();
    assert!(content.contains("priority: low"));
    assert!(content.contains("area: core"));
    assert!(content.contains("status: pending"));
    assert!(content.contains("milestone: v2"));
    assert!(content.contains("labels: [new, urgent]"));
    assert!(content.contains("# JSON Title"));
    assert!(content.contains("json body"));
    assert!(!content.contains("old"));
}

#[test]
fn update_from_json_rejects_invalid_json() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--json"])
        .write_stdin("{")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to parse JSON input"));
}

#[test]
fn update_json_rejects_cli_fields() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--json", "--priority", "high"])
        .write_stdin(r#"{"area":"cli"}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("--json cannot be combined"));
}

#[test]
fn update_json_rejects_unknown_fields() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--json"])
        .write_stdin(r#"{"add_label":["urgent"]}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown field"));
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

#[test]
fn edit_operates_on_misplaced_active_issue_with_warning() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-misplaced.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Misplaced\n",
    )
    .unwrap();

    renga(&dir)
        .env("EDITOR", "true")
        .args(["edit", "1"])
        .assert()
        .success()
        .stderr(predicate::str::contains("stored in done/"));
}

// ── assignee ──────────────────────────────────────────────────────────────────

#[test]
fn create_with_assignee() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--assignee", "alice"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("assignee: alice"));
}

#[test]
fn create_without_assignee_omits_field() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(!content.contains("assignee:"));
}

#[test]
fn update_sets_assignee() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--assignee", "bob"])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("assignee: bob"));
}

#[test]
fn list_filters_by_assignee() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task A", "--assignee", "alice"])
        .assert()
        .success();
    renga(&dir).args(["create", "Task B"]).assert().success();
    let output = renga(&dir)
        .args(["list", "--assignee", "alice"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(out.contains("Task A"));
    assert!(!out.contains("Task B"));
}

#[test]
fn list_json_includes_assignee() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--assignee", "alice"])
        .assert()
        .success();
    renga(&dir)
        .args(["list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"assignee\": \"alice\""));
}

#[test]
fn show_json_includes_assignee() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--assignee", "alice"])
        .assert()
        .success();
    renga(&dir)
        .args(["show", "1", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"assignee\": \"alice\""));
}

#[test]
fn create_json_with_assignee() {
    let dir = setup();
    renga(&dir)
        .args(["create", "--json"])
        .write_stdin(r#"{"title": "Task", "assignee": "alice"}"#)
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("assignee: alice"));
}

#[test]
fn update_clears_assignee() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--assignee", "alice"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--assignee", ""])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(!content.contains("assignee:"));
}

#[test]
fn update_clears_milestone() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--milestone", "v1"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--milestone", ""])
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(!content.contains("milestone:"));
}

#[test]
fn update_json_with_assignee() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--json"])
        .write_stdin(r#"{"assignee": "charlie"}"#)
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(content.contains("assignee: charlie"));
}

#[test]
fn update_json_clears_assignee() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--assignee", "alice"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--json"])
        .write_stdin(r#"{"assignee": ""}"#)
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(!content.contains("assignee:"));
}

#[test]
fn update_json_clears_milestone() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--milestone", "v1"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--json"])
        .write_stdin(r#"{"milestone": ""}"#)
        .assert()
        .success();
    let content = fs::read_to_string(dir.path().join("issues/open/1-task.md")).unwrap();
    assert!(!content.contains("milestone:"));
}

// ── directory-based issues ────────────────────────────────────────────────────

#[test]
fn create_dir_creates_directory_with_readme() {
    let dir = setup();
    renga(&dir)
        .args(["create", "My Issue", "--dir=true"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1-my-issue/README.md"));

    let readme = dir.path().join("issues/open/1-my-issue/README.md");
    assert!(readme.exists());
    let content = fs::read_to_string(&readme).unwrap();
    assert!(content.contains("status: open"));
    assert!(content.contains("# My Issue"));
}

#[test]
fn create_dir_next_id_counts_directory_issues() {
    let dir = setup();
    renga(&dir)
        .args(["create", "First", "--dir=true"])
        .assert()
        .success();
    renga(&dir)
        .args(["create", "Second"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2-second.md"));
}

#[test]
fn update_dir_true_expands_file_to_directory() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--dir=true"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1-task/README.md"));

    assert!(!dir.path().join("issues/open/1-task.md").exists());
    assert!(dir.path().join("issues/open/1-task/README.md").exists());
}

#[test]
fn update_dir_false_collapses_directory_to_file() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--dir=false"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1-task.md"));

    assert!(!dir.path().join("issues/open/1-task").exists());
    assert!(dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn update_dir_false_fails_when_extra_files_present() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    fs::write(dir.path().join("issues/open/1-task/notes.md"), "extra file").unwrap();
    renga(&dir)
        .args(["update", "1", "--dir=false"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("README.md"));
}

#[test]
fn done_moves_directory_issue_to_done() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    renga(&dir)
        .args(["done", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("done/1-task/README.md"));

    assert!(!dir.path().join("issues/open/1-task").exists());
    let readme = dir.path().join("issues/done/1-task/README.md");
    assert!(readme.exists());
    let content = fs::read_to_string(&readme).unwrap();
    assert!(content.contains("status: done"));
}

#[test]
fn list_includes_directory_issues() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Dir Issue", "--dir=true"])
        .assert()
        .success();
    renga(&dir)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Dir Issue"));
}

#[test]
fn show_works_for_directory_issue() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Dir Task", "--dir=true"])
        .assert()
        .success();
    renga(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Dir Task"));
}

#[test]
fn update_status_moves_directory_issue() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--status", "pending"])
        .assert()
        .success()
        .stdout(predicate::str::contains("pending/1-task/README.md"));

    assert!(!dir.path().join("issues/open/1-task").exists());
    assert!(dir.path().join("issues/pending/1-task/README.md").exists());
}

#[test]
fn update_dir_cannot_be_combined_with_other_fields() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    renga(&dir)
        .args(["update", "1", "--dir=true", "--priority", "high"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--dir"));
}

#[test]
fn update_dir_true_already_dir_fails() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--dir=true"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already a directory"));
}

#[test]
fn validate_auto_correct_moves_directory_issue_to_correct_status() {
    let dir = setup();
    renga(&dir)
        .args(["create", "Task", "--dir=true"])
        .assert()
        .success();
    // Manually place the directory in the wrong status folder
    let open_dir = dir.path().join("issues/open/1-task");
    let done_dir = dir.path().join("issues/done/1-task");
    fs::create_dir_all(&done_dir).unwrap();
    fs::rename(&open_dir, &done_dir).unwrap();
    // But frontmatter still says open — validate should correct it
    let readme = done_dir.join("README.md");
    let content = fs::read_to_string(&readme).unwrap();
    // leave frontmatter status as "open" (it was created as open)
    assert!(content.contains("status: open"));

    renga(&dir)
        .args(["validate", "--auto-correct"])
        .assert()
        .success()
        .stdout(predicate::str::contains("corrected:"));

    assert!(dir.path().join("issues/open/1-task/README.md").exists());
    assert!(!dir.path().join("issues/done/1-task").exists());
}

// ── group_by ──────────────────────────────────────────────────────────────────

#[test]
fn create_places_issue_under_area_when_group_by_configured() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success()
        .stdout(predicate::str::contains("issues/core/open/1-task.md"));
    assert!(dir.path().join("issues/core/open/1-task.md").exists());
    assert!(!dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn create_falls_back_to_flat_when_area_empty_under_group_by() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir).args(["create", "Task"]).assert().success();
    assert!(dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn create_rejects_reserved_area_name_under_group_by() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "Done"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("reserved"));
}

#[test]
fn update_status_change_relocates_across_area_bucket() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--status", "pending"])
        .assert()
        .success();
    assert!(dir.path().join("issues/core/pending/1-task.md").exists());
    assert!(!dir.path().join("issues/core/open/1-task.md").exists());
}

#[test]
fn update_area_change_relocates_file() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--area", "backend"])
        .assert()
        .success();
    assert!(dir.path().join("issues/backend/open/1-task.md").exists());
    assert!(!dir.path().join("issues/core/open/1-task.md").exists());
}

#[test]
fn update_rejects_reserved_area_name_under_group_by() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();
    renga(&dir)
        .args(["update", "1", "--area", "Open"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("reserved"));
}

#[test]
fn done_moves_within_area_bucket() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();
    renga(&dir).args(["done", "1"]).assert().success();
    assert!(dir.path().join("issues/core/done/1-task.md").exists());
}

#[test]
fn pending_moves_within_area_bucket() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();
    renga(&dir).args(["pending", "1"]).assert().success();
    assert!(dir.path().join("issues/core/pending/1-task.md").exists());
}

#[test]
fn in_progress_moves_within_area_bucket() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();
    renga(&dir).args(["in-progress", "1"]).assert().success();
    assert!(dir
        .path()
        .join("issues/core/in-progress/1-task.md")
        .exists());
}

#[test]
fn done_multiple_ids_across_different_areas() {
    // Regression guard: the per-status commands used to compute their
    // destination directory once outside the per-id loop. Under group_by
    // each issue's destination depends on its own area, so passing multiple
    // ids from different areas in one invocation must place each correctly.
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "First", "--area", "core"])
        .assert()
        .success();
    renga(&dir)
        .args(["create", "Second", "--area", "backend"])
        .assert()
        .success();

    renga(&dir).args(["done", "1", "2"]).assert().success();

    assert!(dir.path().join("issues/core/done/1-first.md").exists());
    assert!(dir.path().join("issues/backend/done/2-second.md").exists());
}

#[test]
fn reopen_relocates_within_area_bucket() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();
    renga(&dir).args(["done", "1"]).assert().success();
    renga(&dir).args(["reopen", "1"]).assert().success();
    assert!(dir.path().join("issues/core/open/1-task.md").exists());
    assert!(!dir.path().join("issues/core/done/1-task.md").exists());
}

#[test]
fn reopen_rejects_collision_within_area_bucket() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Foo", "--area", "core"])
        .assert()
        .success();
    renga(&dir).args(["done", "1"]).assert().success();
    fs::create_dir_all(dir.path().join("issues/core/open")).unwrap();
    fs::write(
        dir.path().join("issues/core/open/1-foo.md"),
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
fn validate_detects_and_corrects_group_by_mismatch() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();

    renga(&dir)
        .args(["validate"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("status directory mismatch"));

    renga(&dir)
        .args(["validate", "--auto-correct"])
        .assert()
        .success()
        .stdout(predicate::str::contains("corrected:"));

    assert!(dir.path().join("issues/core/open/1-task.md").exists());
    assert!(!dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn validate_auto_correct_ungroups_when_group_by_disabled() {
    let dir = setup();
    fs::create_dir_all(dir.path().join("issues/core/open")).unwrap();
    fs::write(
        dir.path().join("issues/core/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    // no .renga.yml — group_by is off, so the canonical location is flat.

    renga(&dir)
        .args(["validate", "--auto-correct"])
        .assert()
        .success()
        .stdout(predicate::str::contains("corrected:"));

    assert!(dir.path().join("issues/open/1-task.md").exists());
    assert!(!dir.path().join("issues/core/open/1-task.md").exists());
}

#[test]
fn validate_flags_reserved_area_collision_without_correcting() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: done\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();

    renga(&dir)
        .args(["validate", "--auto-correct"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("reserved status directory name"));

    assert!(dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn migrate_relocates_existing_issues_under_new_group_by() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrated 1 issue(s)."));

    assert!(dir.path().join("issues/core/open/1-task.md").exists());
    assert!(!dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn migrate_skips_reserved_collision_area_with_warning() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: done\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stderr(predicate::str::contains("skipping"));

    assert!(dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn migrate_prints_nothing_to_migrate_when_group_by_already_canonical() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--area", "core"])
        .assert()
        .success();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Nothing to migrate."));
}

#[test]
fn info_shows_group_by() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
    let output = renga(&dir).args(["info"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("group_by"));
}

// Regression tests for issue #232: done/pending/in-progress/reopen must
// still operate on issues with unparseable frontmatter (falling back to no
// area), matching their pre-group_by behavior — group_by doesn't even need
// to be configured for this, since these commands never required valid
// frontmatter before.

#[test]
fn done_tolerates_unparseable_frontmatter() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    renga(&dir).args(["done", "1"]).assert().success();
    assert!(dir.path().join("issues/done/1-bad.md").exists());
}

#[test]
fn pending_tolerates_unparseable_frontmatter() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    renga(&dir).args(["pending", "1"]).assert().success();
    assert!(dir.path().join("issues/pending/1-bad.md").exists());
}

#[test]
fn in_progress_tolerates_unparseable_frontmatter() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    renga(&dir).args(["in-progress", "1"]).assert().success();
    assert!(dir.path().join("issues/in-progress/1-bad.md").exists());
}

#[test]
fn reopen_tolerates_unparseable_frontmatter() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/done/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    renga(&dir).args(["reopen", "1"]).assert().success();
    assert!(dir.path().join("issues/open/1-bad.md").exists());
}

#[test]
fn migrate_group_by_step_falls_back_to_unknown_for_unparseable_frontmatter() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-bad.md"),
        "---\nnot: valid: yaml: [\n---\n\n# Bad\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrated 1 issue(s)."));

    assert!(dir.path().join("issues/unknown/1-bad.md").exists());
}

#[test]
fn migrate_group_by_step_skips_destination_collision() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("issues/core/open")).unwrap();
    fs::write(
        dir.path().join("issues/core/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: high\narea: core\nlabels: []\n---\n\n# Task (existing)\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stderr(predicate::str::contains("skipping"));

    let content = fs::read_to_string(dir.path().join("issues/core/open/1-task.md")).unwrap();
    assert!(content.contains("Task (existing)"));
    assert!(dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn migrate_reports_single_count_for_two_hop_relocation() {
    // Regression guard for issue #235: a flat top-level file that hops
    // through both migrate steps (flat -> status -> area/status) must be
    // counted once, not twice.
    let dir = setup();
    fs::write(
        dir.path().join("issues/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrated 1 issue(s)."));

    assert!(dir.path().join("issues/core/open/1-task.md").exists());
}

// ── defaults.dir ──────────────────────────────────────────────────────────────

#[test]
fn create_uses_defaults_dir_from_config() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "defaults:\n  dir: true\n").unwrap();
    renga(&dir)
        .args(["create", "Task"])
        .assert()
        .success()
        .stdout(predicate::str::contains("issues/open/1-task/README.md"));
    assert!(dir.path().join("issues/open/1-task/README.md").exists());
}

#[test]
fn create_explicit_dir_false_overrides_defaults_dir_config() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "defaults:\n  dir: true\n").unwrap();
    renga(&dir)
        .args(["create", "Task", "--dir=false"])
        .assert()
        .success();
    assert!(dir.path().join("issues/open/1-task.md").exists());
    assert!(!dir.path().join("issues/open/1-task").exists());
}

#[test]
fn create_defaults_dir_unset_is_flat_without_flag() {
    let dir = setup();
    renga(&dir).args(["create", "Task"]).assert().success();
    assert!(dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn create_json_input_also_uses_defaults_dir_config() {
    // The --json input path has no `dir` field of its own (there was never
    // a way to request --dir via JSON), so defaults.dir applies uniformly
    // regardless of which input mode created the issue.
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "defaults:\n  dir: true\n").unwrap();
    renga(&dir)
        .args(["create", "--json"])
        .write_stdin(r#"{"title":"JSON Task"}"#)
        .assert()
        .success();
    assert!(dir
        .path()
        .join("issues/open/1-json-task/README.md")
        .exists());
}

#[test]
fn migrate_converts_flat_issues_to_dir_when_defaults_dir_enabled() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(dir.path().join(".renga.yml"), "defaults:\n  dir: true\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrated 1 issue(s)."));

    assert!(dir.path().join("issues/open/1-task/README.md").exists());
    assert!(!dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn migrate_dir_conversion_skips_collision_with_warning() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("issues/open/1-task")).unwrap();
    fs::write(dir.path().join(".renga.yml"), "defaults:\n  dir: true\n").unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stderr(predicate::str::contains("skipping"));

    assert!(dir.path().join("issues/open/1-task.md").exists());
}

#[test]
fn migrate_prints_nothing_to_migrate_when_defaults_dir_already_canonical() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "defaults:\n  dir: true\n").unwrap();
    renga(&dir).args(["create", "Task"]).assert().success();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Nothing to migrate."));
}

#[test]
fn migrate_defaults_dir_and_group_by_together_relocate_to_nested_path() {
    let dir = setup();
    fs::write(
        dir.path().join("issues/open/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(
        dir.path().join(".renga.yml"),
        "defaults:\n  dir: true\ngroup_by: [area]\n",
    )
    .unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrated 1 issue(s)."));

    assert!(dir
        .path()
        .join("issues/core/open/1-task/README.md")
        .exists());
}

#[test]
fn migrate_reports_single_count_for_three_hop_relocation() {
    // Regression guard for issue #235's bug class: a flat top-level file
    // that hops through all three migrate steps (flat -> status ->
    // dir-based -> area/status) must be counted once, not multiple times.
    let dir = setup();
    fs::write(
        dir.path().join("issues/1-task.md"),
        "---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n",
    )
    .unwrap();
    fs::write(
        dir.path().join(".renga.yml"),
        "defaults:\n  dir: true\ngroup_by: [area]\n",
    )
    .unwrap();

    renga(&dir)
        .args(["migrate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Migrated 1 issue(s)."));

    assert!(dir
        .path()
        .join("issues/core/open/1-task/README.md")
        .exists());
}

#[test]
fn info_shows_defaults_dir() {
    let dir = setup();
    fs::write(dir.path().join(".renga.yml"), "defaults:\n  dir: true\n").unwrap();
    let output = renga(&dir).args(["info"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("defaults.dir"));
}
