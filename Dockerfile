FROM nixos/nix as builder-base

RUN nix

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN nix flake update && nix shell -c cargo build --release --workspace

FROM photon as runner-base

ENV RUST_LOG="info" \
    SERVER_PORT=8080

RUN yum update -y && yum upgrade -y

FROM runner-base as runner 

COPY --chown=55 .config /config
VOLUME [ "/config" ]

COPY --from=builder /workspace/target/release/conduit /bin/conduit

FROM runner

EXPOSE 80
EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "app" ]
CMD [ "system", "--up" ]
