# syntax = docker/dockerfile:experimental
FROM clux/muslrust:nightly-2020-04-17 as builder

COPY migrations/ migrations/
COPY src/ src/
COPY templates/ templates/
COPY Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --path .

FROM alpine:3.11

WORKDIR /data

RUN apk add --no-cache ca-certificates tzdata

COPY --from=builder /root/.cargo/bin/amelio /app/

EXPOSE 8080

ENTRYPOINT ["/app/amelio"]
