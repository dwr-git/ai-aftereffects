#!/usr/bin/env bash
# Build the browser version and zip it:  $DIST/ai-aftereffects-web-<version>.zip
#
# Usage: packaging/web/package.sh [--skip-build]
#
# Needs: the wasm32-unknown-unknown target and the wasm-bindgen CLI at the version in Cargo.lock;
# `cargo xtask web` builds it (wasm-opt from binaryen is used when installed). The zip holds a
# self-contained static site in ai-aftereffects-web-<version>/ that works from any URL path.
#
# AI Анимация: the app's WebAssembly is shipped gzipped and cut into parts (app/), listed in
# app/parts.txt; the page (apps/effectcraft-web/web/index.html) downloads and inflates them, shows
# the progress (studio/loader.js reads data-app-bytes) and hands the compiled module to the app and
# its render workers. No file is larger than one part. The service worker of the upstream build is
# left out: every build lives in its own b/<N>/ folder (studio/build/versioned.sh).
set -euo pipefail
# shellcheck source=../env.sh
. "$(dirname "${BASH_SOURCE[0]}")/../env.sh"
HERE="$ROOT/packaging/web"

if [[ "${1:-}" != "--skip-build" ]]; then
  # CI: install the wasm-bindgen CLI at the crate's exact version.
  if [[ -n "${CI:-}" ]]; then
    WASM_BINDGEN_VERSION="$(sed -n 's/^const WASM_BINDGEN: &str = "\(.*\)";$/\1/p' "$ROOT/xtask/src/main.rs")"
    if ! { command -v wasm-bindgen >/dev/null && wasm-bindgen --version | grep -qF "$WASM_BINDGEN_VERSION"; }; then
      cargo install wasm-bindgen-cli --version "$WASM_BINDGEN_VERSION" --locked
    fi
  fi
  (cd "$ROOT" && cargo xtask web)
fi

SITE="$CARGO_TARGET_DIR/web/dist"
[[ -f "$SITE/index.html" ]] || { echo "error: $SITE/index.html missing; run without --skip-build" >&2; exit 1; }
# Paths must be relative so the site works under any prefix.
if grep -Eq '(src|href)="/[^/]' "$SITE/index.html"; then
  echo "error: $SITE/index.html has root-absolute URLs; it would break when served from a sub-path" >&2
  exit 1
fi
WASM="$SITE/effectcraft_web_bg.wasm"
[[ -f "$WASM" ]] || { echo "error: $WASM missing" >&2; exit 1; }
[[ -f "$SITE/effectcraft_web.js" ]] || { echo "error: $SITE/effectcraft_web.js missing" >&2; exit 1; }

NAME="ai-aftereffects-web-$VERSION"
WORK="$CARGO_TARGET_DIR/web-package"
rm -rf "$WORK"
mkdir -p "$WORK/$NAME/app"
cp -R "$SITE/." "$WORK/$NAME/"
# Not shipped: the wasm in one piece (the parts replace it), the upstream offline worker, manifest and icons.
rm -f "$WORK/$NAME/effectcraft_web_bg.wasm" "$WORK/$NAME/sw.js" "$WORK/$NAME/manifest.webmanifest" \
  "$WORK/$NAME/favicon.svg" "$WORK/$NAME/icon-256.png" "$WORK/$NAME/icon-512.png"

# Size limit: some hosts and CDNs reject single files over 25 MiB; parts stay well under it.
PART_BYTES="${AI_AFTEREFFECTS_WEB_PART_BYTES:-20971520}" # 20 MiB
HASH="$(sha256 "$WASM" | cut -c1-16)"
PART_STEM="ai-aftereffects-$HASH"
gzip -9 -n -c "$WASM" | (cd "$WORK/$NAME/app" && split -b "$PART_BYTES" - "$PART_STEM.part-")
{
  echo "ai-aftereffects-parts 1"
  for part in "$WORK/$NAME/app/$PART_STEM".part-*; do basename "$part"; done
} >"$WORK/$NAME/app/parts.txt"
wasm_bytes=$(wc -c <"$WASM" | tr -d ' ')
gzip_bytes=$(cat "$WORK/$NAME/app/$PART_STEM".part-* | wc -c | tr -d ' ')
# The page shows download progress against this total (studio/loader.js).
sed -i.bak "s/data-app-bytes=\"0\"/data-app-bytes=\"$gzip_bytes\"/" "$WORK/$NAME/index.html" && rm -f "$WORK/$NAME/index.html.bak"
grep -q "data-app-bytes=\"$gzip_bytes\"" "$WORK/$NAME/index.html" || { echo "error: the page does not know the download size" >&2; exit 1; }
part_count=$(($(wc -l <"$WORK/$NAME/app/parts.txt") - 1))
echo "effectcraft_web_bg.wasm: $wasm_bytes bytes, shipped as $gzip_bytes bytes of gzip in $part_count part(s) of up to $PART_BYTES"
# Sample server config (MIME types); harmless where unused.
cp "$HERE/.htaccess" "$WORK/$NAME/" 2>/dev/null || true
copy_docs "$WORK/$NAME"
rm -f "$DIST/$NAME.zip"
(cd "$WORK" && zip -qr9 "$DIST/$NAME.zip" "$NAME")
echo "wrote $DIST/$NAME.zip"
ls -lh "$DIST/$NAME.zip"
