#!/usr/bin/env bash
# Build and package the collaboration server for this machine (see docs/server.md):
#
#   $DIST/novafig-server-<version>-<os>-<arch>.tar.gz
#
# One directory with the novafig-server binary, the licences, docs/server.md and the deployment
# files (install.sh, systemd units, Caddyfile, nginx). A headless binary: no desktop files, no fonts.
#
# Usage: packaging/server/package.sh [--skip-build]
set -euo pipefail
# shellcheck source=../env.sh
. "$(dirname "${BASH_SOURCE[0]}")/../env.sh"
HERE="$ROOT/packaging/server"

SKIP_BUILD=0
case "${1:-}" in
  --skip-build) SKIP_BUILD=1 ;;
  "") ;;
  -h | --help) sed -n '2,9p' "$0"; exit 0 ;;
  *) echo "unknown argument: $1" >&2; exit 2 ;;
esac

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
  arm64) ARCH=aarch64 ;;
  amd64) ARCH=x86_64 ;;
esac
NAME="novafig-server-$VERSION-$OS-$ARCH"
echo "==> novafig-server $VERSION for $OS $ARCH"

if [ "$SKIP_BUILD" = 0 ]; then
  (cd "$ROOT" && cargo build --release --locked -p novafig-server)
fi
WORK="$CARGO_TARGET_DIR/server-package"
STAGE="$WORK/$NAME"
rm -rf "$WORK"
mkdir -p "$STAGE"
cp "$CARGO_TARGET_DIR/release/novafig-server" "$STAGE/"
strip "$STAGE/novafig-server" 2>/dev/null || true
for f in LICENSE-MIT LICENSE-APACHE NOTICE docs/server.md; do cp "$ROOT/$f" "$STAGE/"; done
cp "$HERE/novafig-server.service" "$HERE/novafig-backup.service" "$HERE/novafig-backup.timer" "$HERE/Caddyfile" "$HERE/nginx.conf" "$STAGE/"
install -m 755 "$HERE/install.sh" "$STAGE/"

# Smoke test: the binary runs and reports the version being shipped.
"$STAGE/novafig-server" --version | grep -F "$VERSION"

tar -C "$WORK" -czf "$DIST/$NAME.tar.gz" "$NAME"
echo "==> $DIST/$NAME.tar.gz"
