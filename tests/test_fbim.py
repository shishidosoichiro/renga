import json
import subprocess
from pathlib import Path

import pytest

from conftest import make_issue, REPO_ROOT, _cov_run

FBIM = REPO_ROOT / "bin" / "fbim"


def run(*args, cwd):
    return _cov_run(FBIM, list(args), cwd)


# ── create ────────────────────────────────────────────────────────────────────

def test_create_makes_file(project):
    result = run("create", "My Issue", "--slug", "my-issue", cwd=project)
    assert result.returncode == 0
    files = list((project / "issues").glob("*-my-issue.md"))
    assert len(files) == 1


def test_create_5digit_id(project):
    run("create", "First", "--slug", "first", cwd=project)
    files = list((project / "issues").glob("*.md"))
    issue_files = [f for f in files if f.name != "README.md"]
    assert issue_files[0].name.startswith("00001-")


def test_create_increments_id(project):
    run("create", "First", "--slug", "first", cwd=project)
    run("create", "Second", "--slug", "second", cwd=project)
    files = sorted(f for f in (project / "issues").glob("[0-9]*.md"))
    assert files[0].name.startswith("00001-")
    assert files[1].name.startswith("00002-")


def test_create_frontmatter(project):
    run("create", "My Issue", "--slug", "my-issue", "--priority", "high", "--area", "auth", cwd=project)
    content = next((project / "issues").glob("*-my-issue.md")).read_text()
    assert "status: open" in content
    assert "priority: high" in content
    assert "area: auth" in content
    assert "# My Issue" in content


def test_create_with_body(project):
    run("create", "My Issue", "--slug", "my-issue", "--body", "Extra context.", cwd=project)
    content = next((project / "issues").glob("*-my-issue.md")).read_text()
    assert "Extra context." in content


def test_create_4digit_backward_compat(project):
    make_issue(project, "0042-legacy.md")
    run("create", "New", "--slug", "new", cwd=project)
    files = sorted(f for f in (project / "issues").glob("[0-9]*.md"))
    assert any(f.name.startswith("00043-") for f in files)


# ── done ──────────────────────────────────────────────────────────────────────

def test_done_moves_file(project):
    make_issue(project, "00001-test.md")
    result = run("done", "1", cwd=project)
    assert result.returncode == 0
    assert not (project / "issues" / "00001-test.md").exists()
    assert (project / "issues" / "done" / "00001-test.md").exists()


def test_done_sets_status(project):
    make_issue(project, "00001-test.md")
    run("done", "1", cwd=project)
    content = (project / "issues" / "done" / "00001-test.md").read_text()
    assert "status: done" in content


def test_done_4digit_backward_compat(project):
    make_issue(project, "0042-legacy.md")
    result = run("done", "42", cwd=project)
    assert result.returncode == 0
    assert (project / "issues" / "done" / "0042-legacy.md").exists()


def test_done_not_found(project):
    result = run("done", "9999", cwd=project)
    assert result.returncode != 0


# ── pending ───────────────────────────────────────────────────────────────────

def test_pending_sets_status(project):
    make_issue(project, "00001-test.md")
    result = run("pending", "1", cwd=project)
    assert result.returncode == 0
    content = (project / "issues" / "00001-test.md").read_text()
    assert "status: pending" in content


def test_pending_not_found(project):
    result = run("pending", "9999", cwd=project)
    assert result.returncode != 0


# ── reopen ────────────────────────────────────────────────────────────────────

def test_reopen_moves_file(project):
    make_issue(project, "00001-test.md", status="done")
    (project / "issues" / "00001-test.md").rename(project / "issues" / "done" / "00001-test.md")
    result = run("reopen", "1", cwd=project)
    assert result.returncode == 0
    assert (project / "issues" / "00001-test.md").exists()
    assert not (project / "issues" / "done" / "00001-test.md").exists()


def test_reopen_sets_status(project):
    make_issue(project, "00001-test.md", status="done")
    (project / "issues" / "00001-test.md").rename(project / "issues" / "done" / "00001-test.md")
    run("reopen", "1", cwd=project)
    content = (project / "issues" / "00001-test.md").read_text()
    assert "status: open" in content


# ── list ──────────────────────────────────────────────────────────────────────

def test_list_json(project):
    make_issue(project, "00001-test.md", area="auth")
    result = run("list", "--json", cwd=project)
    assert result.returncode == 0
    data = json.loads(result.stdout)
    assert len(data) == 1
    assert data[0]["area"] == "auth"
    assert data[0]["id"] == "00001"


def test_list_excludes_done_by_default(project):
    make_issue(project, "00001-open.md", status="open")
    make_issue(project, "00002-done.md", status="done")
    (project / "issues" / "00002-done.md").rename(project / "issues" / "done" / "00002-done.md")
    data = json.loads(run("list", "--json", cwd=project).stdout)
    assert all(i["status"] != "done" for i in data)


def test_list_filter_area(project):
    make_issue(project, "00001-auth.md", area="auth")
    make_issue(project, "00002-api.md", area="api")
    data = json.loads(run("list", "--area", "auth", "--json", cwd=project).stdout)
    assert len(data) == 1
    assert data[0]["area"] == "auth"


def test_list_4digit_backward_compat(project):
    make_issue(project, "0042-legacy.md")
    data = json.loads(run("list", "--json", cwd=project).stdout)
    assert data[0]["id"] == "0042"


# ── show ──────────────────────────────────────────────────────────────────────

def test_show(project):
    make_issue(project, "00001-test.md", title="My Issue")
    result = run("show", "1", cwd=project)
    assert result.returncode == 0
    assert "# My Issue" in result.stdout


def test_show_not_found(project):
    result = run("show", "9999", cwd=project)
    assert result.returncode != 0


# ── 親ディレクトリ探索 ─────────────────────────────────────────────────────────

def test_create_from_subdirectory(project):
    subdir = project / "src"
    subdir.mkdir()
    result = run("create", "Sub Issue", "--slug", "sub-issue", cwd=subdir)
    assert result.returncode == 0
    assert len(list((project / "issues").glob("*-sub-issue.md"))) == 1


def test_done_from_subdirectory(project):
    make_issue(project, "00001-test.md")
    subdir = project / "src"
    subdir.mkdir()
    result = run("done", "1", cwd=subdir)
    assert result.returncode == 0
    assert (project / "issues" / "done" / "00001-test.md").exists()


def test_no_issues_dir_error(tmp_path):
    result = run("create", "No Dir", "--slug", "no-dir", cwd=tmp_path)
    assert result.returncode != 0
    assert "error" in result.stderr


# ── issues_dir 設定 ────────────────────────────────────────────────────────────

def test_issues_dir_from_config(tmp_path):
    (tmp_path / ".fbim.yml").write_text("issues_dir: tracking\n")
    (tmp_path / "tracking" / "done").mkdir(parents=True)
    result = run("create", "Config Issue", "--slug", "config-issue", cwd=tmp_path)
    assert result.returncode == 0
    assert len(list((tmp_path / "tracking").glob("*-config-issue.md"))) == 1
    assert not (tmp_path / "issues").exists()


def test_issues_dir_from_config_subdirectory(tmp_path):
    (tmp_path / ".fbim.yml").write_text("issues_dir: tracking\n")
    (tmp_path / "tracking" / "done").mkdir(parents=True)
    subdir = tmp_path / "src"
    subdir.mkdir()
    result = run("create", "Deep Issue", "--slug", "deep-issue", cwd=subdir)
    assert result.returncode == 0
    assert len(list((tmp_path / "tracking").glob("*-deep-issue.md"))) == 1
