#!/bin/sh
set -eu

# Host keys live in the container: a restart keeps them, recreating the container
# (`down` then `up`) makes new ones, which is how tests simulate a changed key.
ssh-keygen -A

# sshd refuses an authorized_keys file the user does not own, and a bind mount
# keeps the host's ownership, so the key is copied in with the right owner.
install -d -o sallygate -g sallygate -m 700 /home/sallygate/.ssh
install -o sallygate -g sallygate -m 600 /test-keys/id_ed25519.pub /home/sallygate/.ssh/authorized_keys

exec /usr/sbin/sshd -D -e
