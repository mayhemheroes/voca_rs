FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD ../ /voca
WORKDIR /voca/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /voca/fuzz/target/x86_64-unknown-linux-gnu/release/voca-fuzz /