FROM php:8.0-cli-alpine3.15
WORKDIR /usr/src/php-bsdiff
COPY . .

RUN \
    set -ex && \
    apk update         && \
    apk add clang gcc  && \
    apk add cargo rust && \
    apk add expect     && \
    cargo install cargo-php && \
    ./install-bsdiff.sh
