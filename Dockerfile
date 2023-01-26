FROM rustlang/rust:nightly-slim as builder
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim as runtime

WORKDIR /root/
COPY --from=builder /app/target/release/rust_battlesnake /usr/local/bin/

CMD ["rust_battlesnake"]
