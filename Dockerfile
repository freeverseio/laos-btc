FROM rust:1.83-bookworm AS builder

WORKDIR /usr/src/laos-btc

COPY . .

RUN cargo build --bin laos-btc --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/laos-btc/target/release/laos-btc /usr/local/bin
RUN apt-get update && apt-get install -y openssl

ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

RUN mkdir /data && \
    chown nobody:nogroup /data && \
    chown nobody:nogroup /usr/local/bin/laos-btc

USER nobody:nogroup

ENTRYPOINT ["/usr/local/bin/laos-btc"]
