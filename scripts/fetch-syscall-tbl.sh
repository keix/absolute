#!/usr/bin/env bash
set -euo pipefail

KERNEL_TAG="${KERNEL_TAG:-v6.12}"
DEST_DIR="data"
DEST="$DEST_DIR/linux-x86_64-syscall_64.tbl"

mkdir -p "$DEST_DIR"
curl -fsSL \
  "https://raw.githubusercontent.com/torvalds/linux/${KERNEL_TAG}/arch/x86/entry/syscalls/syscall_64.tbl" \
  -o "$DEST"

echo "downloaded $DEST (linux $KERNEL_TAG)"
