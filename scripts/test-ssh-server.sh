#!/bin/sh
# Starts or stops the local SSH test server (127.0.0.1:2222, user `sallygate`).
# Usage: scripts/test-ssh-server.sh up|down
set -eu

server_dir="$(cd "$(dirname "$0")/../tests/ssh-server" && pwd)"
key_file="$server_dir/.keys/id_ed25519"

case "${1:-up}" in
  up)
    if [ ! -f "$key_file" ]; then
      mkdir -p "$server_dir/.keys"
      ssh-keygen -q -t ed25519 -N '' -C 'sallygate-test' -f "$key_file"
    fi
    docker compose -f "$server_dir/compose.yaml" up -d --build --wait
    echo "SSH test server: ssh -i $key_file -p 2222 sallygate@127.0.0.1"
    ;;
  down)
    docker compose -f "$server_dir/compose.yaml" down
    ;;
  *)
    echo "Usage: $0 up|down" >&2
    exit 1
    ;;
esac
