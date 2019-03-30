FROM rust:slim

COPY . /code
RUN cargo install --path /code && cargo install mdbook

WORKDIR /data
VOLUME ["/data"]

