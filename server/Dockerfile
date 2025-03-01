FROM rust:1.76 as builder

WORKDIR /usr/src/gyst-server
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/gyst-server/target/release/gyst-server /usr/local/bin/gyst-server

ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

EXPOSE 8080

CMD ["gyst-server"]
