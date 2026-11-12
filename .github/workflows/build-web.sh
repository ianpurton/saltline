#!/usr/bin/env bash
# Produce the exact archive consumed by the release and Wavedash workflows.
set -euo pipefail
cd "$(dirname "$0")/../.."
profile="${WEB_PROFILE:-release}"
bindgen_version="$(python3 - <<'PY'
import tomllib
with open('Cargo.lock', 'rb') as f:
    print(next(p['version'] for p in tomllib.load(f)['package'] if p['name'] == 'wasm-bindgen'))
PY
)"
if [[ "$(wasm-bindgen --version)" != "wasm-bindgen $bindgen_version" ]]; then
  echo "Install matching CLI: cargo install wasm-bindgen-cli --version $bindgen_version --locked" >&2
  exit 1
fi
cargo build --locked --profile "$profile" --target wasm32-unknown-unknown -p saltline-game --bin saltline-game
output_profile="$profile"
if [[ "$profile" == dev ]]; then output_profile=debug; fi
# Only generated web files belong in this directory, never reference screenshots.
rm -rf dist/web
mkdir -p dist/web
wasm-bindgen --target web --out-dir dist/web --out-name saltline \
  "${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/$output_profile/saltline-game.wasm"
cp web/index.html dist/web/index.html
cp assets/fonts/OFL.txt dist/web/FONT-LICENSE.txt
python3 - <<'PY'
import hashlib
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED
root = Path('dist/web')
for name in ('index.html', 'saltline.js', 'saltline_bg.wasm', 'FONT-LICENSE.txt'):
    assert (root / name).stat().st_size > 0, f'Missing web file: {name}'
assert (root / 'saltline_bg.wasm').read_bytes()[:8] == b'\0asm\x01\0\0\0'
archive = Path('tmp/saltline-web.zip')
archive.parent.mkdir(parents=True, exist_ok=True)
with ZipFile(archive, 'w', ZIP_DEFLATED) as z:
    for file in sorted(root.rglob('*')):
        if file.is_file():
            z.write(file, file.relative_to(root))
Path('tmp/saltline-web.zip.sha256').write_text(hashlib.sha256(archive.read_bytes()).hexdigest() + '  saltline-web.zip\n')
PY
