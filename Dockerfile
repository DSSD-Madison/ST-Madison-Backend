FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache build-base

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp /app/target/release/main /app/server

# Runtime
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/server ./server

ENV PORT=3000

EXPOSE 3000

CMD ["./server"]
