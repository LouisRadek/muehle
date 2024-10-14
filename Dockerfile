FROM httpd:2.4.62-alpine

COPY ./index.html /usr/local/apache2/htdocs

COPY ./target/wasm32-unknown-unknown/release/muehle.wasm /usr/local/apache2/htdocs/target/wasm32-unknown-unknown/release/muehle.wasm

COPY ./js /usr/local/apache2/htdocs/js
