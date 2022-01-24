FROM rust:1.58-alpine as builder

WORKDIR /volume

RUN apk add --no-cache build-base=~0.5 musl-dev=~1.2 perl=~5.34

COPY assets/ assets/
COPY migrations/ migrations/
COPY src/ src/
COPY templates/ templates/
COPY build.rs Cargo.lock Cargo.toml rust-toolchain ./

RUN cargo build --release && \
    strip --strip-all target/release/amelio

FROM alpine:3.15

RUN apk add --no-cache ca-certificates=~20211220 && \
    addgroup -g 1000 amelio && \
    adduser -u 1000 -G amelio -D -g '' -H -h /dev/null -s /sbin/nologin amelio

COPY --from=builder /volume/target/release/amelio /bin/

EXPOSE 8080
USER amelio

ENTRYPOINT ["/bin/amelio"]
