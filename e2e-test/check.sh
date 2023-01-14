#!/usr/bin/env bash

set -eu -o pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
PROJECT_DIR=$(realpath $SCRIPT_DIR/..)
DATA_DIR="$SCRIPT_DIR/testdata"

cargo build
BIN="$PROJECT_DIR/target/debug/gzip"

rm -f "$DATA_DIR/*.gz"

FILES=$( ls $DATA_DIR )

for FILE in $FILES
do
    TEMP_FILE="$SCRIPT_DIR/temp"
    "$BIN" "$DATA_DIR/$FILE"
    gunzip -c "$DATA_DIR/$FILE.gz" > "$TEMP_FILE"
    rm "$DATA_DIR/$FILE.gz" 
    diff "$DATA_DIR/$FILE" "$TEMP_FILE"
    rm "$TEMP_FILE"
done
