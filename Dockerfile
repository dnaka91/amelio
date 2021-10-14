FROM rust:1.55 as builder

WORKDIR /volume

RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools=1.2.2-1 && \
    rustup toolchain add nightly-2021-08-01 && \
    rustup target add --toolchain nightly-2021-08-01 x86_64-unknown-linux-musl

COPY assets/ assets/
COPY migrations/ migrations/
COPY src/ src/
COPY templates/ templates/
COPY build.rs Cargo.lock Cargo.toml rust-toolchain ./

RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip --strip-all target/x86_64-unknown-linux-musl/release/amelio

FROM alpine:3.14

RUN apk add --no-cache ca-certificates=20191127-r5 && \
    addgroup -g 1000 amelio && \
    adduser -u 1000 -G amelio -D -g '' -H -h /dev/null -s /sbin/nologin amelio

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/amelio /bin/

EXPOSE 8080
USER amelio

ENTRYPOINT ["/bin/amelio"]
