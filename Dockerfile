ARG RUST_VERSION=1.78.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

WORKDIR /app

COPY . .

RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/taxum /

FROM debian:bookworm-slim AS runner-builder

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

COPY --from=builder /taxum /usr/local/bin

RUN chown appuser /usr/local/bin/taxum

COPY --from=builder /app/.config /opt/taxum/.config
COPY --from=builder /app/assets /opt/taxum/assets

RUN chown -R appuser /opt/taxum

USER appuser

ENV RUST_LOG="taxum=debug,info"

WORKDIR /opt/taxum

ENTRYPOINT ["taxum"]

EXPOSE 8080/tcp