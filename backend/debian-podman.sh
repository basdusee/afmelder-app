# This script starts a Docker container (well, podman, which is the same)
# with Rust inside to compile an actix REST back-end.
# Do a `cd /opt/afmelder/ && cargo build --release` or something like that inside the container.
# Reason for this set-up is that the Debian VPS has an older glibc than my laptop.
# I could do all kinds of cross-compile shenanigans, or just compile it in a Debian docker.
#
# Literally replace the word `podman` with `docker` if you have docker installed instead of podman.
# all the arguments are identical.
# podman can run rootless and docker can't, so next to te security problems involved with docker,
# all your files compiled inside the container will be owned by root and not your user, which is less optimal.
#
# Yes smarty pants, you can replace /bin/bash for "cd /opt/afmelder && cargo build --release" 
# and make it a complete integral experience. But this still is a quick hack
# Put it in yourself if you're so smart.
#
# Why do I put more effort in the text of this script than the actual script?
# I do not have a proper answer to that, my friend.
#
podman run \
-it \
--name afmelderbuilder \
-v "$(pwd)":/opt/afmelder \
rust:slim-bullseye \
/bin/bash
