# king-of-the-castle
Rust implementation of the vintage board game "Kráľovské rošády"

## Run without docker
To run the application, run:
```
cargo run --bin server
cargo run --bin client
```

## Run with Docker
TODO

## Run client
To run webassembly client:
Initial setup:
```
cargo install trunk
rustup target add wasm32-unknown-unknown
trunk serve --open
```
