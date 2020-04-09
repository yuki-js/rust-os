FROM ubuntu:latest

RUN apt update && apt upgrade -y
RUN apt install -y curl nasm mtools git openssh-client tar gzip ca-certificates openssh-server build-essential
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup install nightly && rustup default nightly && rustup target add i686-unknown-linux-gnu && rustup override set nightly