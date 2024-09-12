# 使用官方 Rust 镜像作为基础镜像
FROM rust:latest

# 设置工作目录
WORKDIR /usr/src/app

# 安装 cargo-watch
RUN cargo install cargo-watch

# 安装psql 
RUN apt-get update && apt-get install -y postgresql-client

# 复制 Cargo.toml 和 Cargo.lock 并构建依赖项
COPY Cargo.toml Cargo.lock ./

# 创建一个虚拟的 main.rs 来预先构建依赖
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build && \
    cargo build --release && \
    rm -rf src

