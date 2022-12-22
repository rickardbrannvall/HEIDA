# HEIDA

1. assumes code is compiled and run in docker container: jupyter/datascience-notebook (or similar)
2. install all in requirements.txt

alternatively you can setup manually in ubuntu with at least jupyterlab installed
(although there may be further requirements)

RUSTFLAGS="-C target-cpu=native" cargo run --release
RUSTFLAGS="-C target-cpu=native" cargo build --release

cargo build
cargo build —release

