#!/usr/bin/env bash

set -e

echo "⚙️ Initializing WASM build environment ..."

rustup default stable

rustup target add wasm32-unknown-unknown --toolchain stable


echo "✅ Done"