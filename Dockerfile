FROM rust:1.72.1 AS builder
COPY src src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /target/release/navigation_server /target/release/navigation_server
COPY src src
ENTRYPOINT ["/target/release/navigation_server"]
