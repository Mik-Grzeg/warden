ARG DEBUG=

FROM rust:1.72-bookworm as builder


WORKDIR /app

COPY ["warden/Cargo.lock", "warden/Cargo.toml"]

RUN mkdir -p src && touch src/main.rs && cargo fetch && rm -rf src

COPY . warden/src/
RUN cargo build

FROM debian:bookworm-slim AS runtime

WORKDIR /APP
COPY --from=builder /app/target/debug/warden /app/warden

ENTRYPOINT [ "/app/warden" ]
