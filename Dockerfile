ARG RUST_VERSION=1.78.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

WORKDIR /app

COPY . .

RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/tapp /

FROM debian:bookworm-slim AS runner-builder

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    postgresql

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

COPY --from=builder /tapp /usr/local/bin

RUN chown appuser /usr/local/bin/tapp

COPY --from=builder /app/.config /opt/tapp/.config
COPY --from=builder /app/assets /opt/tapp/assets

RUN chown -R appuser /opt/tapp

USER appuser

ENV DATABASE_URI \ RUST_LOG="tapp=debug,info"

WORKDIR /opt/tapp

ENTRYPOINT ["tapp"]

EXPOSE 8080/tcp