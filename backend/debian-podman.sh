# This script starts a Docker container (well, podman, which is the same)
# with Rust inside to compile an actix REST back-end.
# Do a `cd /opt/afmelder/ && cargo build` or something like that inside the container.
# Reason for this set-up is that the Debian VPS has an older glibc than my laptop.
# I could do all kinds of cross-compile shenanigans, or just compile it in a Debian docker.
podman run \
-it \
--name afmelderbuilder \
-v "$(pwd)":/opt/afmelder \
rust:slim-bullseye \
/bin/bash
