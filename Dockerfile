FROM rust:1.62 AS builder

WORKDIR /usr/app

COPY /src ./src
COPY Cargo.toml .

RUN cargo build

FROM rust:1.62

WORKDIR /usr/app

COPY --from=builder /usr/app/target ./target

CMD ["./target/debug/bitcoin-strategies"]


