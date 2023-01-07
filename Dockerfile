FROM nixos/nix as builder-base

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN nix --extra-experimental-features nix-command --extra-experimental-features flakes flake update
RUN nix --extra-experimental-features nix-command --extra-experimental-features flakes shell -c cargo build --release --workspace

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
