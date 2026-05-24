#!/usr/bin/env bash
# Build the macOS binary locally and upload it to the GitLab Package Registry.
#
# Usage:
#   ./upload-macos.sh                   # uploads as "main" version
#   VERSION=v0.2.0 ./upload-macos.sh    # uploads as a specific version
#
# Requires:
#   - Rust toolchain (cargo)
#   - GITLAB_TOKEN env var, or glab authenticated to gitlab.home
set -euo pipefail

GITLAB_URL="${GITLAB_URL:-https://gitlab.home}"
PROJECT_PATH="${PROJECT_PATH:-kiwi/ifbm}"
VERSION="${VERSION:-main}"

GITLAB_TOKEN="${GITLAB_TOKEN:-$(glab config get token --host "${GITLAB_URL#https://}" 2>/dev/null || true)}"
if [[ -z "$GITLAB_TOKEN" ]]; then
  echo "error: GITLAB_TOKEN is not set and glab is not authenticated" >&2
  exit 1
fi

ENCODED_PROJECT=$(python3 -c "import urllib.parse; print(urllib.parse.quote('${PROJECT_PATH}', safe=''))")
UPLOAD_URL="${GITLAB_URL}/api/v4/projects/${ENCODED_PROJECT}/packages/generic/fbim/${VERSION}/fbim-darwin-arm64"

echo "Building macOS binary..."
cargo build --release

echo "Uploading to ${GITLAB_URL}/${PROJECT_PATH} (version: ${VERSION})..."
curl --fail -s --upload-file target/release/fbim \
  -H "PRIVATE-TOKEN: ${GITLAB_TOKEN}" \
  "$UPLOAD_URL"
echo "Uploaded: fbim-darwin-arm64 → ${VERSION}"
