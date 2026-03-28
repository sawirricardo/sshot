#!/bin/sh
set -e

REPO="sawirricardo/sshot"
INSTALL_DIR="/usr/local/bin"
BINARY="sshot"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)  os="linux" ;;
    Darwin) os="darwin" ;;
    *)      echo "Unsupported OS: $OS" >&2; exit 1 ;;
esac

case "$ARCH" in
    x86_64|amd64)  arch="x86_64" ;;
    arm64|aarch64) arch="aarch64" ;;
    *)             echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

# Get latest release tag
echo "Fetching latest release..."
tag=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$tag" ]; then
    echo "Error: could not determine latest release" >&2
    exit 1
fi

echo "Installing sshot ${tag} for ${os}/${arch}..."

# Download
url="https://github.com/${REPO}/releases/download/${tag}/sshot-${os}-${arch}.tar.gz"
tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

curl -sL "$url" -o "${tmpdir}/sshot.tar.gz"
tar -xzf "${tmpdir}/sshot.tar.gz" -C "$tmpdir"

# Install
if [ -w "$INSTALL_DIR" ]; then
    mv "${tmpdir}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
else
    echo "Need sudo to install to ${INSTALL_DIR}"
    sudo mv "${tmpdir}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
fi

chmod +x "${INSTALL_DIR}/${BINARY}"

echo "Installed sshot ${tag} to ${INSTALL_DIR}/${BINARY}"
