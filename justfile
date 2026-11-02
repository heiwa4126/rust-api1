default:
    cargo run

test:
    cargo test

build:
    cargo build --release

lint:
    cargo clippy && cargo fmt --check && just --fmt --check --unstable

fmt:
    cargo fmt && oxfmt && just --fmt --unstable

update:
    # `cargo install cargo-edit`
    cargo upgrade --incompatible

audit:
    # `cargo install cargo-audit`
    cargo audit

release:
    goreleaser build --snapshot --single-target --clean
