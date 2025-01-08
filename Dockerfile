FROM ubuntu:22.04

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    qemu-system \
    && rm -rf /var/lib/apt/lists/*

# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY rust-toolchain.toml .
RUN rustup show

RUN rustup target add x86_64-unknown-none

CMD ["/bin/bash"]
