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
DUCKDB_DOWNLOAD_LIB=1 cargo run
```

6. Run tests

```bash
DUCKDB_DOWNLOAD_LIB=1 cargo test
```

## Code Quality

```bash
DUCKDB_DOWNLOAD_LIB=1 cargo fmt
DUCKDB_DOWNLOAD_LIB=1 cargo clippy
DUCKDB_DOWNLOAD_LIB=1 cargo test
```

## Parcel Tiles

This repo includes a static parcel tile pipeline for the full `silver.parcels` dataset
(82,152 parcels). It extracts parcel geometries from DuckDB/GCS, writes newline-delimited
GeoJSON, and builds a single PMTiles archive for MapLibre.

### Local Build

Prerequisites:
- `duckdb` CLI
- `tippecanoe`
- `GCS_KEY_ID` and `GCS_SECRET` in your environment or `.env`

Run:

```bash
set -a
source .env
set +a
./scripts/generate_parcel_pmtiles.sh
```

Output:
- `build/tiles/parcels.geojson.ndjson`
- `build/tiles/madison-parcels.pmtiles`

Defaults:
- minimum zoom: `10` (citywide Madison view)
- maximum zoom: `14`

Override those if needed:

```bash
MIN_ZOOM=9 MAX_ZOOM=15 ./scripts/generate_parcel_pmtiles.sh
```

### GitHub Action

The scheduled workflow is in `.github/workflows/parcel-tiles.yml`. It runs weekly and can
also be triggered manually.

Required GitHub secrets:
- `GCS_KEY_ID`
- `GCS_SECRET`
- `CLOUDFLARE_R2_ACCESS_KEY_ID`
- `CLOUDFLARE_R2_SECRET_ACCESS_KEY`

Required GitHub repository variables:
- `CLOUDFLARE_R2_BUCKET`
- `CLOUDFLARE_R2_ENDPOINT`

Optional GitHub repository variable:
- `CLOUDFLARE_R2_OBJECT_KEY` (defaults to `madison-parcels.pmtiles`)

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
