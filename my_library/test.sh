#!/bin/bash
cargo test
cargo test --no-default-features --features xorshift
cargo test --no-default-features --features pcg
