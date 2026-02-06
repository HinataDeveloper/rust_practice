#! /bin/bash

clear
cargo check && cargo build && cargo run --color=always --package Test --bin Test --profile dev -- $1 $2 $3
ls src/
