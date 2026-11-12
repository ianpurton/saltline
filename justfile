# List local development commands.
default:
    @just --list

# Build the browser release and release archive (WEB_PROFILE=dev for debugging).
build-web:
    .github/workflows/build-web.sh

# Build and serve the browser game locally; stop with Ctrl+C.
run-web port="8000": build-web
    python3 -m http.server {{ quote(port) }} --bind 127.0.0.1 --directory dist/web
