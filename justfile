
bounds:
    mkdir -p data/bounds/
    cargo run --features cli lower > data/bounds/lower.tsv
    cargo run --features cli mid > data/bounds/mid.tsv
    cargo run --features cli upper > data/bounds/upper.tsv
