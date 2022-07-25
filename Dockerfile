FROM rust:1.60-alpine as builder

WORKDIR /volume

RUN apk add --no-cache build-base=~0.5 musl-dev=~1.2 perl=~5.34

COPY assets/ assets/
COPY migrations/ migrations/
COPY src/ src/
COPY templates/ templates/
COPY build.rs Cargo.lock Cargo.toml rust-toolchain ./

RUN cargo build --release

FROM alpine:3.16.1 as newuser

RUN echo "amelio:x:1000:" > /tmp/group && \
    echo "amelio:x:1000:1000::/dev/null:/sbin/nologin" > /tmp/passwd

FROM scratch

COPY --from=builder /volume/target/release/amelio /bin/
COPY --from=newuser /tmp/group /tmp/passwd /etc/

EXPOSE 8080
USER amelio

ENTRYPOINT ["/bin/amelio"]
