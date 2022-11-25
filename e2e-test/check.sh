#!/usr/bin/env bash

set -eu -o pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DATA_DIR="$SCRIPT_DIR/testdata"

FILES=$( ls $DATA_DIR )

for FILE in $FILES
do
    TEMP_FILE="$SCRIPT_DIR/temp"
    cat "$DATA_DIR/$FILE" | cargo run | gunzip - > "$TEMP_FILE"
    diff "$DATA_DIR/$FILE" "$TEMP_FILE"
    rm "$TEMP_FILE"
done
