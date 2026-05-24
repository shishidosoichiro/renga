import subprocess
from pathlib import Path

from conftest import make_issue, REPO_ROOT, _cov_run

GEN = REPO_ROOT / "bin" / "gen-issues-readme"


def run(cwd, *args):
    return _cov_run(GEN, list(args), cwd)


def test_generates_readme(project):
    make_issue(project, "00001-test.md", title="Test Issue", area="auth")
    result = run(project)
    assert result.returncode == 0
    readme = (project / "issues" / "README.md").read_text()
    assert "Test Issue" in readme
    assert "00001" in readme


def test_stdout_flag(project):
    make_issue(project, "00001-test.md", title="Test Issue")
    result = run(project, "--stdout")
    assert result.returncode == 0
    assert "Test Issue" in result.stdout
    assert not (project / "issues" / "README.md").exists()


def test_excludes_pending_by_default(project):
    make_issue(project, "00001-open.md", title="Open Issue", status="open")
    make_issue(project, "00002-pending.md", title="Pending Issue", status="pending")
    result = run(project, "--stdout")
    assert "Open Issue" in result.stdout
    assert "Pending Issue" in result.stdout


def test_excludes_done_by_default(project):
    make_issue(project, "00001-open.md", title="Open Issue")
    make_issue(project, "00002-done.md", title="Done Issue", status="done")
    (project / "issues" / "00002-done.md").rename(project / "issues" / "done" / "00002-done.md")
    result = run(project, "--stdout")
    assert "Open Issue" in result.stdout
    assert "Done Issue" not in result.stdout


def test_filter_by_area(project):
    make_issue(project, "00001-auth.md", title="Auth Issue", area="auth")
    make_issue(project, "00002-api.md", title="API Issue", area="api")
    result = run(project, "--area", "auth", "--stdout")
    assert "Auth Issue" in result.stdout
    assert "API Issue" not in result.stdout


def test_area_order_from_config(project):
    make_issue(project, "00001-b.md", title="B Issue", area="b")
    make_issue(project, "00002-a.md", title="A Issue", area="a")
    (project / ".fbim.yml").write_text("area_order:\n  - b\n  - a\n")
    result = run(project, "--stdout")
    b_pos = result.stdout.index("B Issue")
    a_pos = result.stdout.index("A Issue")
    assert b_pos < a_pos


def test_area_alphabetical_without_config(project):
    make_issue(project, "00001-z.md", title="Z Issue", area="z")
    make_issue(project, "00002-a.md", title="A Issue", area="a")
    result = run(project, "--stdout")
    a_pos = result.stdout.index("A Issue")
    z_pos = result.stdout.index("Z Issue")
    assert a_pos < z_pos


def test_area_labels_from_config(project):
    make_issue(project, "00001-auth.md", title="Auth Issue", area="auth")
    (project / ".fbim.yml").write_text("area_labels:\n  auth: 'Authentication'\n")
    result = run(project, "--stdout")
    assert "Authentication" in result.stdout


def test_4digit_backward_compat(project):
    make_issue(project, "0042-legacy.md", title="Legacy Issue")
    result = run(project, "--stdout")
    assert "Legacy Issue" in result.stdout
    assert "0042" in result.stdout


# ── 親ディレクトリ探索 ─────────────────────────────────────────────────────────

def test_generates_from_subdirectory(project):
    make_issue(project, "00001-test.md", title="Sub Test")
    subdir = project / "sub"
    subdir.mkdir()
    result = run(subdir, "--stdout")
    assert result.returncode == 0
    assert "Sub Test" in result.stdout


# ── issues_dir 設定 ────────────────────────────────────────────────────────────

def test_issues_dir_from_config(tmp_path):
    (tmp_path / ".fbim.yml").write_text("issues_dir: tracking\n")
    (tmp_path / "tracking" / "done").mkdir(parents=True)
    (tmp_path / "tracking" / "00001-test.md").write_text(
        "---\nstatus: open\npriority: medium\narea: misc\nlabels: []\n---\n\n# Config Test\n"
    )
    result = run(tmp_path, "--stdout")
    assert result.returncode == 0
    assert "Config Test" in result.stdout
