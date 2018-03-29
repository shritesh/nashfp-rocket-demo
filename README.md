# Rocket.rs Demo for NashFP

## Install Rust nightly
```
curl https://sh.rustup.rs -sSf | sh
rustup default nightly
```

## Hello World Demo
```
cargo run --bin hello_world
```

## Authentication demo

### Install diesel-cli with postgresql support
```
cargo install diesel_cli --no-default-features --features postgres
```

### Postgresql
Ensure postgresql is running and the `DATABASE_URL` is configured in `.env` file

### Initialize the database
Look at the migrations directory for table structure and seed data.
```
diesel setup
```

### Run the demo
```
cargo run --bin demo
```