# king-of-the-castle
Rust implementation of the vintage board game "Kráľovské rošády"

## Install and run yew web server
```shell
cargo install trunk
rustup target add wasm32-unknown-unknown
cd client && trunk server --open
```

## Run without docker
To run the application, run:
```
cargo install diesel_cli --no-default-features --features postgres
(cd server/kotc_database && diesel migration run)
cargo run --bin server

cargo run --bin client
```

## Run with Docker
TODO
