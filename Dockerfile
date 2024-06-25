FROM rust:latest as builder-base

RUN apt-get update -y && \
    apt-get upgrade -y && \
    rustup update

FROM builder-base as builder

ADD . /workspace

WORKDIR /workspace

COPY . .

RUN cargo build -r -v

FROM debian:latest as runner-base

ENV RUST_LOG=info \
    SERVER_PORT=8080 

RUN apt-get update -y && apt-get upgrade -y

COPY --from=builder /workspace/target/release/pzzld /app/template-rs-api
COPY --from=builder /workspace/assets /app/assets
COPY --from=builder /workspace/backend.config.toml /app/backend.config.toml

EXPOSE 8080

ENTRYPOINT [ "app/template-rs-api" ]
