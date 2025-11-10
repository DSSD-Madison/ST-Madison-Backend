# Strong Towns Madison Backend

## Prerequisites
- Rust setup
    - [Install Rust](https://rust-lang.org/tools/install/)
    - [Setup Rust-Analyzer](https://rust-analyzer.github.io/)
    - [Setup rustfmt](https://github.com/rust-lang/rustfmt)
    - [Setup clippy](https://github.com/rust-lang/rust-clippy)
- Install Docker
- [Rust (book)](https://doc.rust-lang.org/book/)

## Getting Started

1. Clone the repo

2. Copy `.env.example` to `.env` and update values

3. Start the database

```bash
# todo
```

4. Run migrations

```bash
# todo
```

5. Start the dev server

```bash
cargo run
```

6. Run tests

```bash
cargo test
```

## Code Quality

```bash
cargo fmt
cargo clippy
cargo test
```

## Tech Stack

- [Axum](https://docs.rs/axum/latest/axum/)
- [Tokio](https://tokio.rs/)
- [ThisError](https://docs.rs/thiserror/latest/thiserror/)
- [Serde](https://crates.io/crates/serde), [Serde JSON](https://docs.rs/serde_json/latest/serde_json/)
- [Tower Middleware](https://docs.rs/tower-http/latest/tower_http/)
- [env_logger](https://docs.rs/env_logger/latest/env_logger/)
- [iter_tools](https://docs.rs/itertools/latest/itertools/)
- [chrono](https://docs.rs/chrono/latest/chrono/)
- [dotenvy](https://docs.rs/dotenvy/0.15.7/dotenvy/)
- [sqlx](https://docs.rs/sqlx/latest/sqlx/)
- [validator](https://docs.rs/validator/latest/validator/)
- [bcrypt](https://docs.rs/bcrypt/latest/bcrypt/)
- [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/)
- [uuid](https://docs.rs/uuid/latest/uuid/)
