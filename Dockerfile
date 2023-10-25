FROM rustlang/rust:nightly-slim as builder
RUN apt update && apt install -y libssl-dev pkg-config
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim as runtime

WORKDIR /root/
COPY --from=builder /app/target/release/battlesnake-rust /usr/local/bin/

CMD ["battlesnake-rust"]
