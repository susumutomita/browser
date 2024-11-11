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

# 環境変数の設定
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# rust-toolchain.tomlをコピー
COPY rust-toolchain.toml .

# Rustツールチェインのインストール
RUN rustup show

CMD ["/bin/bash"] 
