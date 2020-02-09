FROM rust:1

RUN mkdir /app && \
  apt-get update && apt-get install -y ffmpeg libcairo2-dev && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./static /app/static

RUN mkdir -p /app/src && touch /app/src/lib.rs
WORKDIR /app
RUN cargo build --release

# Include default assets
RUN mkdir /app/tmp
COPY ./assets/IPAexfont00401 /app/tmp/IPAexfont00401

COPY ./src /app/src
RUN cargo build --release

CMD cargo run --release
