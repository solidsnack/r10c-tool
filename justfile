
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

# Assert that Cargo.toml, typst/typst.toml and a release tag all agree.
check-version tag="":
    #!/bin/bash
    set -o errexit -o nounset -o pipefail

    # Called by CI on a tag push, and by `tag` below. The tag is optional, so
    # that this also serves as a plain "do the two files agree?" check.

    crate="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
    package="$(sed -n 's/^version = "\(.*\)"/\1/p' typst/typst.toml | head -1)"

    if [ "$crate" != "$package" ]; then
        echo "Cargo.toml is $crate but typst/typst.toml is $package" >&2
        exit 1
    fi

    tag="{{tag}}"
    if [ -n "$tag" ] && [ "${tag#v}" != "$crate" ]; then
        echo "tag $tag does not match version $crate" >&2
        exit 1
    fi

    echo "$crate"

# Tag a release, once the versions agree. Push separately to release.
tag version="":
    #!/bin/bash
    set -o errexit -o nounset -o pipefail

    # The version defaults to the one in Cargo.toml; passing one explicitly
    # only asserts it is that version -- bump the files first, then tag.

    version="{{version}}"
    version="${version#v}"

    if [ -z "$version" ]; then
        version="$(just check-version)"
    else
        just check-version "v$version" > /dev/null
    fi

    tag="v$version"

    if git rev-parse --verify --quiet "refs/tags/$tag" > /dev/null; then
        echo "$tag already exists" >&2
        exit 1
    fi

    if [ -n "$(git status --porcelain)" ]; then
        echo "working tree is dirty; commit before tagging" >&2
        exit 1
    fi

    git tag --annotate --message "r10c $version" "$tag"
    echo "created $tag at $(git rev-parse --short HEAD); push to release:"
    echo "    git push origin $tag"
