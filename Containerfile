FROM docker.io/rustlang/rust:nightly AS builder
WORKDIR /app
RUN cargo install cargo-deb

COPY Cargo.toml Cargo.lock /app
RUN cargo fetch
COPY src /app/src
COPY reporters.json watchers.json LICENSE /app
COPY debian /app/debian
RUN cargo deb

FROM scratch AS package
COPY --from=builder /app/target/debian /
