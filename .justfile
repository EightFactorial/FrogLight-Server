#!/usr/bin/env just --justfile

# Alias for `run`
default: (build-profile "dev")

# ---- Build Recipes ----

# Compile development build
alias build := build-profile
# Compile development build
alias build-dev := build-profile
# Compile release build
build-release: (build-profile "release" "")

# Compile build with specified profile
[private]
build-profile profile="dev" args="":
  cargo build --bin froglight-server --profile {{profile}} {{args}}

# Run development build
alias run := run-profile
# Run development build
alias run-dev := run-profile
# Run release build
run-release: (run-profile "release" "")

# Run build with specified profile
[private]
run-profile profile="dev" args="":
  cargo run --bin froglight-server --profile {{profile}} --features=mimalloc {{args}}

# Clean build artifacts
clean:
  cargo clean

# ---- Test Recipes ----

# Run all tests and all tool tests
all-tests: (update) (deny) (fmt) (test) (graph)

# Run all tests and doc-tests
test: (nextest) (doc-test) 

# Run all tests
nextest:
  cargo nextest run --workspace

# Get number of threads
threads := `nproc --all`

# Run all doc-tests
# Uses at most 4 threads
doc-test: 
  cargo test --doc --workspace -- --test-threads=$(( {{threads}} > 4 ? 4 : {{threads}} ))

# ---- Tool Recipes ----

# Run cargo deny
deny:
  cargo deny check

# Run cargo update
update:
  cargo update

# Run clippy
clippy:
  cargo clippy --workspace

# Run cargo fmt
fmt:
  cargo fmt --all

# Generate froglight system graphs
graph:
  RUST_LOG=info cargo run --example system-graph

