#!/usr/bin/env just --justfile

default:
    @just test
    @just clippy
    @just fmt
    @just run

run:
  cargo run

check:
    cargo check --all-features

clippy:
    cargo clippy --all-targets --all-features -- -D clippy::all

fmt:
    cargo fmt --all -- --check

test: build
    cargo test .

build: check
    cargo build --release
