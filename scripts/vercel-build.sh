#!/usr/bin/env sh

set -eu

# Vercel's Rust toolchain is exposed from /rust rather than ~/.cargo.
if [ -f /rust/env ]; then
  . /rust/env
fi

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "Installing wasm-pack..."
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  if [ -f /rust/env ]; then
    . /rust/env
  elif [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
  fi
fi

exec wasm-pack build --target web --release
