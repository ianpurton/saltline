# Saltline releases and Wavedash

The workflows build the `saltline-game` binary in this workspace. Browser builds
use Bevy's WebGL 2 backend and the embedded Oswald font. They require a
WebGL 2-capable browser with hardware acceleration enabled. Serve the build over
HTTPS (or localhost for local testing).

Browser cameras use forward LDR rendering without HDR, bloom, screen-space
reflections, contact shadows, volumetric fog, temporal anti-aliasing, or generated
environment lighting. Skyboxes, distance fog, ambient/direct lighting and ordinary
shadows remain enabled. Native builds retain their existing rendering effects.
There are no fixture downloads or external game-data folders.

## Configure GitHub once

Create a GitHub environment named `wavedash`. In that environment configure:

- Secret `WAVEDASH_TOKEN`: a Wavedash CLI token with access to the Saltline game.
- Variable `WAVEDASH_GAME_ID`: the Saltline project's game ID, not the ID from the
  project these workflow files originally came from.

Optional environment protection rules can restrict who deploys or publishes.
Release workflows use the repository's `GITHUB_TOKEN` with `contents: write`.
Deploy uses `contents: read` to download the selected stable release.

## Release flow

1. Push changes and let **Build and validate** finish. It runs native tests,
   packages the Linux executable, and builds the actual browser archive.
2. On `main`, run **Release candidate**. It calculates `vX.Y.Z-rc.N`, builds the
   browser ZIP, then attaches it and its SHA-256 checksum to a GitHub prerelease.
3. Run **Release**, choosing that RC (blank chooses the newest RC tag). It promotes
   the exact same archive to `vX.Y.Z` without rebuilding it.
4. Run **Deploy Wavedash**. Leave version blank for GitHub's latest stable release,
   or specify an existing `vX.Y.Z`. It verifies the checksum and uploads the build.
5. With **publish** left unchecked, the build is available through the workflow's
   playtest URL. Test startup, menu, pointer lock, and gameplay there. Check
   **publish** on a deployment run to also make that uploaded build live.

Artifacts are consistently named `saltline-web.zip` and
`saltline-web.zip.sha256`. The ZIP root contains `index.html`, `saltline.js`,
`saltline_bg.wasm`, generated bindings, and `FONT-LICENSE.txt`. Reference images,
source code, and native binaries are not uploaded to Wavedash.

## Build locally

Install the browser target and the wasm-bindgen CLI version selected by Cargo.lock:

```bash
rustup target add wasm32-unknown-unknown
version=$(python3 -c 'import tomllib; print(next(p["version"] for p in tomllib.load(open("Cargo.lock", "rb"))["package"] if p["name"] == "wasm-bindgen"))')
cargo install wasm-bindgen-cli --version "$version" --locked
scripts/build-web.sh
python3 -m http.server 8000 --directory dist/web
```

Open `http://localhost:8000`. `WEB_PROFILE=dev scripts/build-web.sh` is also
supported for debugging. CI and release candidates always use the release profile.
The CLI version check prevents incompatible JS/WASM bindings after lockfile updates.

The deployment command and environment configuration follow the official
[Wavedash CLI reference](https://docs.wavedash.com/cli/commands) and
[environment-variable reference](https://docs.wavedash.com/cli/environment-variables).
A local browser build cannot verify account permissions, Wavedash processing,
or the final hosted iframe; the uploaded playtest remains the final deployment check.

## Verification for this adaptation

WebGL 2 migration verified locally with Bevy 0.20.1:

- Native workspace check, all seven workspace tests, formatting and workflow validation pass.
- The release browser build, archive CRCs and SHA-256 checksum pass.
- The WASM feature graph enables `webgl2` and contains no `webgpu` feature.
- Chromium with WebGPU disabled and `navigator.gpu` unavailable uses the AMD GPU
  through the WebGL 2 backend. Title rendering, clicking Deploy, pointer lock,
  movement/look/fire/reload and pause/resume were exercised without shader or GPU
  validation errors. Captures are in `tmp/screenshots/webgl-{title,deployed,paused,resumed}.png`.
- A browser with WebGL 2 unavailable displays the expected startup error.

Performance on the affected user's hardware has not been measured; the backend
switch and reduced effects do not establish a frame-rate improvement by themselves.

Earlier checks, performed before the WebGL 2 migration:

- `actionlint` validates all four workflow files.
- `cargo fmt --all --check` and all seven `cargo test --locked --workspace` tests pass.
- WASM checking and the exact release `scripts/build-web.sh` build pass.
- The release archive's checksum, ZIP CRCs, and required entrypoints verify.
- Chromium with a Vulkan WebGPU adapter renders the title and enters gameplay
  through Deploy. Captures are in `tmp/screenshots/web-title.png` and
  `tmp/screenshots/web-deployed.png`. Software SwiftShader lost its GPU device
  in this container; it was not counted as a successful render check.
- Installed Wavedash CLI help confirms the upload/publish flags used here.

GitHub Actions runners, GitHub release publication, and authenticated Wavedash
uploads have not been executed from this local verification session.
