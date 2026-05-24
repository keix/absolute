#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

cd "$REPO_ROOT"
cargo lambda build --release --arm64 --bin system-calls

cd "$REPO_ROOT/infra"
if [[ ! -d node_modules ]]; then
  npm install
fi

npx cdk deploy
