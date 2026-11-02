#!/bin/sh
# Runs every local check in order and stops at the first failure.
# Usage: scripts/check-all.sh [--integration]
set -eu

for check in typecheck lint format:check test check:architecture lint:rust test:rust; do
  echo "==> $check"
  bun run "$check"
done

if [ "${1:-}" = "--integration" ]; then
  echo "==> test:integration"
  bun run test:integration
fi

echo "All checks passed."
