FROM php:8.0-cli-alpine3.15
WORKDIR /usr/src/php-bsdiff
COPY . .

RUN \
    set -ex && \
    apk update && \
    apk add clang clang-dev clang-static gcc g++ build-base libc-dev && \
    apk add cargo expect rust && \
    cargo install cargo-php && \
    ./install-bsdiff.sh
