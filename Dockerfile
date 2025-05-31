FROM rustlang/rust:nightly
LABEL stage=intermediate
WORKDIR /app
RUN cargo install cargo-deb
COPY ./Cargo.lock ./Cargo.toml /app/
COPY ./src /app/src

CMD ["cargo-deb"]
