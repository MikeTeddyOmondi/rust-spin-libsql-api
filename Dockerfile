FROM scratch
COPY spin.toml /spin.toml
COPY target/wasm32-wasi/release/spin_rust_api.wasm /target/wasm32-wasi/release/spin_rust_api.wasm
ENTRYPOINT ["/spin.toml"] 

