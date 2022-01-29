# king-of-the-castle
Rust implementation of the vintage board game "Kráľovské rošády"

## Run without docker
To run the application, run:
```
cargo install diesel_cli --no-default-features --features postgres
(cd server/kotc_database && cargo migration run)
cargo run --bin server

cargo run --bin client
```

## Run with Docker
TODO
