FROM rust:latest AS builder
RUN rustup toolchain install nightly && rustup default nightly
RUN cargo install cargo-fuzz

ADD ../ /voca
WORKDIR /voca/fuzz
RUN RUSTFLAGS='-A unexpected_cfgs' cargo +nightly fuzz build

FROM ubuntu:24.04
COPY --from=builder /voca/fuzz/target/x86_64-unknown-linux-gnu/release/voca-fuzz /
