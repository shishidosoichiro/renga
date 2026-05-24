import os
import pytest
from pathlib import Path


REPO_ROOT = Path(__file__).parent.parent

# Enable subprocess coverage: subprocesses inherit COVERAGE_PROCESS_START and
# PYTHONPATH so that sitecustomize.py at the repo root can call coverage.process_startup().
os.environ.setdefault("COVERAGE_PROCESS_START", str(REPO_ROOT / ".coveragerc"))
os.environ.setdefault("COVERAGE_FILE", str(REPO_ROOT / ".coverage"))
_pythonpath = str(REPO_ROOT)
if "PYTHONPATH" in os.environ:
    os.environ["PYTHONPATH"] = _pythonpath + os.pathsep + os.environ["PYTHONPATH"]
else:
    os.environ["PYTHONPATH"] = _pythonpath


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
