FROM rust:latest AS builder
WORKDIR /app
RUN apt update && apt install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cloudflare-ddns-ipv6 /
CMD ["/cloudflare-ddns-ipv6"]