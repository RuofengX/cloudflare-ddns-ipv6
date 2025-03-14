# 阶段1：依赖计算（使用 cargo-chef 工具）
FROM rust:slim AS planner
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 阶段2：依赖缓存构建
FROM rust:slim AS cacher
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 阶段3：应用编译（启用静态链接）
FROM rust:slim AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
# 安装 musl 工具链并编译为静态二进制
RUN apt-get update && apt-get install -y musl-tools
RUN RUSTUP_DIST_SERVER="https://rsproxy.cn" \ 
    RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
    rustup target add x86_64-unknown-linux-musl \
    && cargo build --target x86_64-unknown-linux-musl --release

# 阶段4：最小化运行时镜像
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cloudflare-ddns-ipv6 /
CMD ["/cloudflare-ddns-ipv6"]