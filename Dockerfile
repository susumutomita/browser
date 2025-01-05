FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    qemu-system \
    && rm -rf /var/lib/apt/lists/*

# Rustのインストール (対話なし、yesで)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 環境変数を通す
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# (オプション) rust-toolchain.toml をコピーする
# もし日付付きnightlyを消して "nightly" とだけ書いてあるならそれでもOK
COPY rust-toolchain.toml .

# 最新のプロファイル最小化 & 最新nightlyをインストール
# もし rust-toolchain.toml があるなら "RUN rustup show" だけでもいいですが
RUN rustup set profile minimal
RUN rustup install nightly-aarch64-unknown-linux-gnu
RUN rustup default nightly-aarch64-unknown-linux-gnu

# 必要なら x86_64-unknown-none とか wasm32-unknown-unknown とか追加
RUN rustup target add x86_64-unknown-none

# 標準ライブラリのソースが必要なら
RUN rustup component add rust-src --toolchain nightly-aarch64-unknown-linux-gnu

CMD ["/bin/bash"]
