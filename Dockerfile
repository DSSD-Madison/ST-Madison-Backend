FROM rust:slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential wget unzip && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN wget https://github.com/duckdb/duckdb/releases/download/v1.4.3/libduckdb-linux-amd64.zip && \
    unzip libduckdb-linux-amd64.zip -d libduckdb

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    DUCKDB_LIB_DIR=/app/libduckdb \
    DUCKDB_INCLUDE_DIR=$DUCKDB_LIB_DIR \
    LD_LIBRARY_PATH=$DUCKDB_LIB_DIR \
    cargo build --release && \
    cp /app/target/release/main /app/server

# Runtime
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/server ./server
COPY --from=builder /app/libduckdb/libduckdb.so /usr/lib/

ENV PORT=3000

EXPOSE 3000

CMD ["./server"]
