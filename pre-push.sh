#!/usr/bin/env bash

set -eu -o pipefail

cargo test
cargo build

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
$SCRIPT_DIR/e2e-test/check.sh
