FROM rust:1.61.0-slim-bullseye

ARG TARGETPLATFORM

ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8 \
    SQLDEF_VERSION=v0.11.60

RUN apt-get -y -q update \
  && apt-get install -y -q \
    libpq-dev \
    libssl-dev \
    pkg-config \
    curl \
  && echo "install sqldef" \
  && SQLDEF_ARCH=$(echo $TARGETPLATFORM | sed -e 's/\//_/') \
  && echo $SQLDEF_ARCH} \
  && curl -L -O https://github.com/k0kubun/sqldef/releases/download/${SQLDEF_VERSION}/psqldef_${SQLDEF_ARCH}.tar.gz \
  && tar xf psqldef_${SQLDEF_ARCH}.tar.gz \
  && rm psqldef_${SQLDEF_ARCH}.tar.gz \
  && mv psqldef /usr/local/bin \
  \
  && cargo install cargo-watch cargo-make

RUN cargo new --bin app
WORKDIR /app
COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

RUN cargo build --color never && \
    rm src/*.rs
