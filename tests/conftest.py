import os
import subprocess
import sys
import pytest
from pathlib import Path


REPO_ROOT = Path(__file__).parent.parent


def _cov_run(script: Path, args: list, cwd) -> subprocess.CompletedProcess:
    """Run a Python script under coverage, writing data to the repo root."""
    cmd = [
        sys.executable, "-m", "coverage", "run",
        "--parallel-mode",
        f"--rcfile={REPO_ROOT / '.coveragerc'}",
        f"--data-file={REPO_ROOT / '.coverage'}",
        str(script),
    ] + list(args)
    return subprocess.run(cmd, capture_output=True, text=True, cwd=cwd)


def make_issue(issues_dir: Path, name: str, title: str = "Test Issue",
               status: str = "open", priority: str = "medium", area: str = "misc") -> Path:
    path = issues_dir / "issues" / name
    path.write_text(
        f"---\nstatus: {status}\npriority: {priority}\narea: {area}\nlabels: []\n---\n\n# {title}\n",
        encoding="utf-8",
    )
    return path


@pytest.fixture
def project(tmp_path):
    (tmp_path / "issues" / "done").mkdir(parents=True)
    return tmp_path
