#!/usr/bin/env bash
# Install fbim by downloading the latest binary from GitLab CI.
#
# Usage:
#   ./install.sh                        # install to /usr/local/bin (default)
#   INSTALL_DIR=~/.local/bin ./install.sh
#
# For private GitLab instances, set GITLAB_TOKEN:
#   GITLAB_TOKEN=<pat> ./install.sh
set -euo pipefail

GITLAB_URL="${GITLAB_URL:-https://gitlab.home}"
PROJECT_PATH="${PROJECT_PATH:-kiwi/ifbm}"
BRANCH="${BRANCH:-main}"
JOB_NAME="${JOB_NAME:-build}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

ARTIFACT_URL="${GITLAB_URL}/${PROJECT_PATH}/-/jobs/artifacts/${BRANCH}/raw/target/release/fbim?job=${JOB_NAME}"

CURL_ARGS=(-fsSL)
if [[ -n "${GITLAB_TOKEN:-}" ]]; then
  CURL_ARGS+=(-H "PRIVATE-TOKEN: ${GITLAB_TOKEN}")
fi

TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT

echo "Downloading fbim from ${GITLAB_URL}/${PROJECT_PATH} (${BRANCH})..."
curl "${CURL_ARGS[@]}" "$ARTIFACT_URL" -o "$TMP"

install -m 0755 "$TMP" "${INSTALL_DIR}/fbim"
echo "Installed: ${INSTALL_DIR}/fbim ($(${INSTALL_DIR}/fbim --version))"
