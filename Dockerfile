FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

# Runtime
FROM alpine:latest

WORKDIR /app

COPY --from=builder /target/release/main ./server

ENV PORT=3000

EXPOSE 3000

CMD ["./server"]
