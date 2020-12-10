#!/bin/bash

cd "$(dirname "$0")"/../crossbeam-epoch
set -ex

export RUSTFLAGS="-D warnings --cfg=loom_crossbeam"

# With MAX_PREEMPTIONS=2 the loom tests (currently) take around 11m.
# If we were to run with =3, they would take several times that,
# which is probably too costly for CI.
env LOOM_MAX_PREEMPTIONS=2 cargo test --test loom --features sanitize --release -- --nocapture
