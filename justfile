
build:
    cargo build --release --target wasm32-unknown-unknown;
    cp ./target/wasm32-unknown-unknown/release/bob_typ.wasm ./typst-package