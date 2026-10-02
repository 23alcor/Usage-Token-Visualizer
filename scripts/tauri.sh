#!/bin/sh

if ! command -v cargo >/dev/null 2>&1 && [ -x /opt/homebrew/opt/rustup/bin/cargo ]; then
  PATH="/opt/homebrew/opt/rustup/bin:$PATH"
  export PATH
fi

exec npx tauri "$@"
