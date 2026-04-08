
lint:
    cargo +nightly fmt --check

fmt:
    cargo +nightly fmt

install:
    cargo install --features cli --path ./

bounds:
    mkdir -p data/bounds/
    cargo run --features cli lower > data/bounds/lower.tsv
    cargo run --features cli mid > data/bounds/mid.tsv
    cargo run --features cli upper > data/bounds/upper.tsv

test:
    cargo test -- --show-output

build:
    cargo build

wasm target="bundler":
    #!/bin/bash
    set -o errexit -o nounset -o pipefail

    mkdir -p tmp/{{target}}
    wasm-pack build --target {{target}} --out-dir tmp/{{target}} \
                    --no-default-features --features f64,wasm
