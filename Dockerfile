FROM rustlang/rust:nightly-slim as builder
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim as runtime

WORKDIR /root/
COPY --from=builder /app/target/release/battlesnake-rust /usr/local/bin/

CMD ["battlesnake-rust"]
