
lint:
    cargo +nightly fmt --check

fmt:
    cargo +nightly fmt

install:
    cargo install --features cli --path ./

run *args:
    cargo run --features cli -- {{args}}

bounds:
    mkdir -p data/bounds/
    cargo run --features cli lower > data/bounds/lower.tsv
    cargo run --features cli mid > data/bounds/mid.tsv
    cargo run --features cli upper > data/bounds/upper.tsv

test:
    cargo test -- --show-output

test-special:
    cargo +nightly test --features f16,f128

build:
    cargo build

build-cli:
    cargo build --release --features cli

build-special:
    cargo +nightly build --features f16,f128
    cargo +nightly build --features cli,f16,f128
    just build-typst
    just build-wasm
    just build-wasm deno

build-typst:
    cargo build --release --features typst --target wasm32-unknown-unknown

build-wasm target="bundler":
    #!/bin/bash
    set -o errexit -o nounset -o pipefail

    mkdir -p tmp/{{target}}
    wasm-pack build --target {{target}} --out-dir tmp/{{target}} \
                    --no-default-features --features f64,wasm
