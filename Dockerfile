# 使用 Rust 官方提供的 `rust` 镜像作为基础镜像
FROM rust:latest as builder

# 安装 musl 工具链
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y musl-tools
# 安装 protoc
RUN apt-get install -y protobuf-compiler

# 创建一个新目录来存放 Rust 项目
WORKDIR /usr/src/myapp

# 复制 Rust 项目中的 Cargo.toml 和 Cargo.lock 文件
COPY Cargo.toml Cargo.lock ./

# 复制 migration 文件夹的 Cargo.toml 文件
COPY migration/Cargo.toml migration/

# 为第一次构建预热 Cargo 依赖项
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir migration/src && \
    echo "fn main() {}" > migration/src/main.rs && \
    echo "fn main() {}" > src/entity/mod.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# 复制整个 Rust 项目源代码到 Docker 镜像中
COPY . .

# 最终构建 Rust 项目
RUN cargo build --release --target x86_64-unknown-linux-musl

# 构建最终的发布镜像
FROM alpine:latest

# 创建一个新目录来存放最终的可执行文件
WORKDIR /usr/app

# 从之前构建的镜像中复制可执行文件到最终镜像中
COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-musl/release/octwords_back .

# 设置容器启动时执行的命令
CMD ["./octwords_back"]
