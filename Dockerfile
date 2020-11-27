# syntax = docker/dockerfile:experimental
FROM clux/muslrust:nightly-2020-11-25 as builder

COPY assets/ assets/
COPY migrations/ migrations/
COPY src/ src/
COPY templates/ templates/
COPY build.rs Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --path .

FROM alpine:3.12

WORKDIR /data

RUN apk add --no-cache ca-certificates

COPY --from=builder /root/.cargo/bin/amelio /app/

EXPOSE 8080

ENTRYPOINT ["/app/amelio"]
