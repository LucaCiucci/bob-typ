# bob-draw-typ
 svgbob for typst, powered by wasm

See [`typst-package/README.md`](typst-package/README.md) for more information.

## Build
```sh
cargo build --release --target wasm32-unknown-unknown; cp ./target/wasm32-unknown-unknown/release/bob_typ.wasm ./typst-package
```