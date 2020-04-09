FROM ubuntu:latest

RUN apt update && apt upgrade -y && apt install -y curl nasm mtools
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup install nightly && rustup default nightly && rustup target add i686-unknown-linux-gnu && rustup override set nightly