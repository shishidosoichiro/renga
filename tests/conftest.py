import pytest
from pathlib import Path


REPO_ROOT = Path(__file__).parent.parent


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
