#!/usr/bin/env sh
# todo: need to check this is a file
SRC_FILE="$1"
TMP_FILE=$(mktemp -u)
# Get the file that changed and pipe to stdin, let leptosfmt fix the views and then pass to rustfmt for further tidying
# save to tmp file as writing over the original in the same command will destroy the file
# todo: check whether leptofmt exists
leptosfmt -s -r < "$SRC_FILE" > "$TMP_FILE"
# move the now tidied file back over the original one.
mv "$TMP_FILE" "$SRC_FILE"