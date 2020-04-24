# Rust API JWT (Draft)

## Setup

Create `.env`:

```
touch .env
```

Populate `.env`:

```
echo "DATABASE_URL=dev.db" >> .env
```

Run:

```
cargo install diesel_cli --no-default-features --features sqlite
```

```
cargo run
```