# syntax = docker/dockerfile:experimental
FROM clux/muslrust:nightly-2021-08-01 as builder

WORKDIR /volume

COPY assets/ assets/
COPY migrations/ migrations/
COPY src/ src/
COPY templates/ templates/
COPY build.rs Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --path .

RUN strip --strip-all /root/.cargo/bin/amelio

FROM alpine:3.14

RUN apk add --no-cache ca-certificates

COPY --from=builder /root/.cargo/bin/amelio /bin/

EXPOSE 8080

ENTRYPOINT ["/bin/amelio"]
