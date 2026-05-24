#!/usr/bin/env bash
# Install fbim from the GitLab Package Registry.
#
# Usage:
#   ./install.sh                        # install to /usr/local/bin (default)
#   INSTALL_DIR=~/.local/bin ./install.sh
#   VERSION=v0.2.0 ./install.sh         # install a specific version
#
# For private GitLab instances, set GITLAB_TOKEN:
#   GITLAB_TOKEN=<pat> ./install.sh
set -euo pipefail

GITLAB_URL="${GITLAB_URL:-https://gitlab.home}"
PROJECT_PATH="${PROJECT_PATH:-kiwi/ifbm}"
VERSION="${VERSION:-main}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

OS=$(uname -s)
ARCH=$(uname -m)
case "${OS}-${ARCH}" in
  Linux-x86_64)   FILE="fbim-linux-x86_64" ;;
  Darwin-arm64)   FILE="fbim-darwin-arm64" ;;
  Darwin-x86_64)  FILE="fbim-darwin-arm64" ;;  # Rosetta
  *)
    echo "error: unsupported platform: ${OS}-${ARCH}" >&2
    exit 1
    ;;
esac

ENCODED_PROJECT=$(python3 -c "import urllib.parse; print(urllib.parse.quote('${PROJECT_PATH}', safe=''))")
ENCODED_VERSION=$(python3 -c "import urllib.parse; print(urllib.parse.quote('${VERSION}', safe=''))")
DOWNLOAD_URL="${GITLAB_URL}/api/v4/projects/${ENCODED_PROJECT}/packages/generic/fbim/${ENCODED_VERSION}/${FILE}"

CURL_ARGS=(-fsSL)
if [[ -n "${GITLAB_TOKEN:-}" ]]; then
  CURL_ARGS+=(-H "PRIVATE-TOKEN: ${GITLAB_TOKEN}")
fi

TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT

echo "Downloading fbim (${VERSION}, ${FILE})..."
curl "${CURL_ARGS[@]}" "$DOWNLOAD_URL" -o "$TMP"

install -m 0755 "$TMP" "${INSTALL_DIR}/fbim"
echo "Installed: ${INSTALL_DIR}/fbim ($("${INSTALL_DIR}/fbim" --version))"
