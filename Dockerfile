FROM rust:1.81-slim as builder

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo build --target wasm32-unknown-unknown --release

FROM httpd:2.4.62-alpine

COPY ./index.html /usr/local/apache2/htdocs

COPY --from=builder ./target/wasm32-unknown-unknown/release/muehle.wasm /usr/local/apache2/htdocs/target/wasm32-unknown-unknown/release/muehle.wasm

COPY ./js /usr/local/apache2/htdocs/js
