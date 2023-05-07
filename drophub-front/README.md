# drophub-front

Frontend (wasm) part of `drophub`

## Init

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.

Use [Trunk] to start the development server. This is a WASM web application bundler for Rust.

JS dependencies are installed via `npm`.

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
npm install
```

## Build

```bash
# Dev build
trunk build
# Release build
trunk build --release
```

## Running

```bash
trunk serve
```

[trunk]: https://github.com/thedodd/trunk
