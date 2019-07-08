FROM rustlang/rust:nightly
LABEL stage=intermediate
WORKDIR /app
COPY ./Cargo.lock ./Cargo.toml /app/
COPY ./src /app/src

CMD ["cargo", "build", "--release"]
