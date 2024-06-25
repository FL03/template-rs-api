ARG RUST_VERSION=1.78.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

WORKDIR /app

COPY . .

RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/template-rs-api /

FROM debian:bookworm-slim AS final

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

COPY --from=builder /template-rs-api /usr/local/bin

RUN chown appuser /usr/local/bin/template-rs-api

COPY --from=builder /app/.config /opt/template-rs-api/.config
COPY --from=builder /app/assets /opt/template-rs-api/assets

RUN chown -R appuser /opt/template-rs-api

USER appuser

ENV RUST_LOG="template_rs_api=debug,info"

WORKDIR /opt/template-rs-api
ENTRYPOINT ["template-rs-api"]
EXPOSE 8080/tcp