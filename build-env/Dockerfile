FROM rust:alpine

RUN apk update && apk upgrade && apk add pkgconfig gcc musl-dev make perl gcc libressl-dev libcrypto3

WORKDIR /build

ENTRYPOINT ["cargo", "build", "--target", "x86_64-unknown-linux-musl", "--release"]