FROM rust:slim

COPY . /code
RUN cargo install --path /code && cargo install mdbook && cargo install mdbook-epub

WORKDIR /data
VOLUME ["/data"]

