import subprocess
from pathlib import Path

NEXT_ID = Path(__file__).parent.parent / "bin" / "next-id"


def run(dir_path):
    return subprocess.run([NEXT_ID, str(dir_path)], capture_output=True, text=True)


def test_empty_dir(tmp_path):
    d = tmp_path / "issues"
    d.mkdir()
    assert run(d).stdout.strip() == "00001"


def test_4digit_files(tmp_path):
    d = tmp_path / "issues"
    d.mkdir()
    (d / "0042-legacy.md").touch()
    assert run(d).stdout.strip() == "00043"


def test_5digit_files(tmp_path):
    d = tmp_path / "issues"
    d.mkdir()
    (d / "00100-test.md").touch()
    assert run(d).stdout.strip() == "00101"


def test_mixed_4_and_5_digit(tmp_path):
    d = tmp_path / "issues"
    d.mkdir()
    (d / "0010-old.md").touch()
    (d / "00050-new.md").touch()
    assert run(d).stdout.strip() == "00051"


def test_scans_done_subdir(tmp_path):
    d = tmp_path / "issues"
    done = d / "done"
    done.mkdir(parents=True)
    (d / "00010-test.md").touch()
    (done / "00050-done.md").touch()
    assert run(d).stdout.strip() == "00051"


def test_missing_dir(tmp_path):
    result = run(tmp_path / "nonexistent")
    assert result.returncode != 0
    assert "error" in result.stderr


def test_no_args():
    result = subprocess.run([NEXT_ID], capture_output=True, text=True)
    assert result.returncode != 0
    assert "usage" in result.stderr
