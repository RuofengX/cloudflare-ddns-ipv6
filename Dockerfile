FROM rust:slim AS planner
WORKDIR /app
COPY . .
RUN cargo install cargo-chef --locked
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:slim AS cacher
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:slim AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cloudflare-ddns-ipv6 /
CMD ["/cloudflare-ddns-ipv6"]