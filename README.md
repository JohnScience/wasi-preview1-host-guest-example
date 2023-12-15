# A workspace WASI example of host-guest interaction

This example demonstrates how to embed `wasmtime` and run a WASI module.

Before executing the `guest` module, the `host` application will also display the imports and exports of the `guest` module.

## Building the binaries

```console
cargo build host
cargo build guest
```

## Running the example

```console
cargo run --bin host ./target/wasm32-wasi/debug/guest.wasm
```
