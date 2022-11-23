# EVM Indexer

> Minimalistic EVM-compatible blockchain indexer written in rust.

This repository contains a program to parse all blocks and transactions into a PostgreSQL database. It also includes all the transaction receipts and logs for contract execution.

## Disclaimer

This program is highly experimental and not meant to be used for production.

## Install

You can try the indexer locally or through Docker.

### Local

To use the program locally, make sure you have [rust](https://www.rust-lang.org/tools/install) installed in your environment.

1. Clone the repository

```bash
git clone https://github.com/eabz/evm-indexer && cd evm-indexer
```

2. Build the program

```bash
cargo build --release
```

3. Copy the .env.example file to .env and add your environment variables.

```
DATABASE_URL -> URL for postgresql database.
RPC_WS_URL -> Websocket URL for the EVM-blockchain RPC endpoint (for new blocks).
RPC_HTTPS_URL -> HTTP URL for the EVM-blockchain RPC endpoint (to fetch past blocks).
```

4. Run the program

```bash
./target/release/evm-indexer
```

### Docker

The code has a builtin docker file that you can build through it, or you can use the constructed automatically image from

```bash
docker pull ghcr.io/eabz/evm-indexer:latest
```

For docker-compose, you can use

```bash
docker-compose up
```

### Database Structure

The database are built using the migrations on the code through the [diesel](https://crates.io/crates/diesel) crate.

Migrations are executed each time the program is started.

There are 5 tables created on the database

- [Blocks (blocks)](#blocks-table)
- [Transactions (txs)](#transactions-table)
- [Transactions Receipts (txs_receipts)](#transactions-receipts-table)
- [Logs (logs)](#logs-table)

#### Blocks Table

| Column             | PostgreSQL type | Rust type |
| ------------------ | --------------- | --------- |
| `number `          | `BIGINT`        | `i64 `    |
| `hash`             | `VARCHAR `      | `String ` |
| `difficulty`       | `VARCHAR `      | `String ` |
| `total_difficulty` | `VARCHAR `      | `String ` |
| `miner`            | `VARCHAR `      | `String ` |
| `gas_limit`        | `VARCHAR `      | `String ` |
| `gas_used`         | `VARCHAR `      | `String ` |
| `txs`              | `BIGINT `       | `i64 `    |
| `timestamp`        | `VARCHAR `      | `String ` |
| `size`             | `VARCHAR `      | `String ` |
| `nonce`            | `VARCHAR `      | `String ` |
| `base_fee_per_gas` | `VARCHAR `      | `String ` |

#### Transactions Table

| Column                     | PostgreSQL type | Rust type |
| -------------------------- | --------------- | --------- |
| `hash `                    | `VARCHAR`       | `String ` |
| `block_number`             | `BIGINT `       | `i64 `    |
| `from_address`             | `VARCHAR `      | `String ` |
| `to_address`               | `VARCHAR `      | `String ` |
| `value`                    | `VARCHAR `      | `String ` |
| `gas_used`                 | `VARCHAR `      | `String ` |
| `gas_price`                | `VARCHAR `      | `String ` |
| `transaction_index`        | `BIGINT `       | `i64 `    |
| `transaction_type`         | `VARCHAR `      | `String ` |
| `max_fee_per_gas`          | `VARCHAR `      | `String ` |
| `max_priority_fee_per_gas` | `VARCHAR `      | `String ` |
| `input`                    | `VARCHAR `      | `String ` |

#### Transactions Receipts Table

| Column    | PostgreSQL type | Rust type |
| --------- | --------------- | --------- |
| `hash `   | `VARCHAR`       | `String ` |
| `success` | `BOOLEAN `      | `bool `   |

#### Logs Table

| Column                  | PostgreSQL type | Rust type      |
| ----------------------- | --------------- | -------------- |
| `hash `                 | `VARCHAR`       | `String `      |
| `address`               | `VARCHAR `      | `String `      |
| `topics`                | `text[] `       | `Vec<String> ` |
| `data`                  | `VARCHAR `      | `String `      |
| `log_index`             | `BIGINT `       | `i64 `         |
| `transaction_log_index` | `BIGINT `       | `i64 `         |
| `log_type`              | `VARCHAR `      | `String `      |
