#!/usr/bin/env bash
set -euo pipefail

REPO="snatvb/imp"
BINARY="imp"

info() { printf "\033[1;34m[imp]\033[0m %s\n" "$1"; }
ok()  { printf "\033[1;32m[imp]\033[0m %s\n" "$1"; }
err() { printf "\033[1;31m[imp]\033[0m %s\n" "$1" >&2; }

version="${1:-}"

if [ -z "$version" ]; then
  info "Resolving latest release..."
  version=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
  if [ -z "$version" ]; then
    err "Could not determine latest version"
    exit 1
  fi
fi

info "Installing ${version}"

os=$(uname -s)
arch=$(uname -m)

case "$os" in
  Linux)  os="linux" ;;
  Darwin) os="mac" ;;
  *)      err "Unsupported OS: $os"; exit 1 ;;
esac

case "$arch" in
  x86_64|amd64)  arch="x64" ;;
  aarch64|arm64) arch="arm64" ;;
  *)             err "Unsupported architecture: $arch"; exit 1 ;;
esac

url="https://github.com/${REPO}/releases/download/${version}/${BINARY}-${version}-${os}-${arch}.tar.gz"
install_dir="${HOME}/.local/bin"
archive="$(mktemp)"

cleanup() { rm -f "$archive"; }
trap cleanup EXIT

info "Downloading ${url}"
curl -fsSL "$url" -o "$archive"

mkdir -p "$install_dir"
info "Installing to ${install_dir}/${BINARY}"
tar xz -C "$install_dir" -f "$archive"

chmod +x "${install_dir}/${BINARY}"

added=false
case ":$PATH:" in
  *":${install_dir}:"*) ;;
  *)
    for rc in "${HOME}/.bashrc" "${HOME}/.zshrc" "${HOME}/.profile"; do
      if [ -f "$rc" ]; then
        echo '' >> "$rc"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
        added=true
        break
      fi
    done
    ;;
esac

if [ "$added" = true ]; then
  info "Added ${install_dir} to PATH (restart your shell)"
fi

info "Verifying installation..."
if "${install_dir}/${BINARY}" --version; then
  ok "Installed successfully"
else
  err "Verification failed"
  exit 1
fi
