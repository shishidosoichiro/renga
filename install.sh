#!/usr/bin/env bash
# Install fbim from GitHub Releases.
#
# Usage:
#   bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/fbim/main/install.sh)
#   INSTALL_DIR=~/.local/bin bash <(curl -fsSL ...)
#   VERSION=v0.5.0 bash <(curl -fsSL ...)
set -euo pipefail

REPO="shishidosoichiro/fbim"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

if [[ -z "${VERSION:-}" ]]; then
  VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' \
    | sed 's/.*"tag_name": *"\(.*\)".*/\1/')
fi

OS=$(uname -s)
ARCH=$(uname -m)
case "${OS}-${ARCH}" in
  Linux-x86_64)   TARGET="x86_64-unknown-linux-gnu" ;;
  Darwin-arm64)   TARGET="aarch64-apple-darwin" ;;
  Darwin-x86_64)  TARGET="aarch64-apple-darwin" ;;  # Rosetta
  *)
    echo "error: unsupported platform: ${OS}-${ARCH}" >&2
    exit 1
    ;;
esac

FILE="fbim-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILE}"

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "Downloading fbim ${VERSION} (${TARGET})..."
curl -fsSL "${URL}" | tar xz -C "$TMP"

install -m 0755 "$TMP/fbim" "${INSTALL_DIR}/fbim"
echo "Installed: ${INSTALL_DIR}/fbim ($("${INSTALL_DIR}/fbim" --version))"
