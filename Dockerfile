FROM rust:1.93.0-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

ENV HEVY_HTTP_ADDR=0.0.0.0:5000
EXPOSE 5000

COPY --from=builder /app/target/release/hevy-mcp-server /usr/local/bin/hevy-mcp-server

ENTRYPOINT ["hevy-mcp-server"]
