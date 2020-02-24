FROM rust:1

RUN mkdir /app && \
  apt-get update && apt-get install -y ffmpeg libcairo2-dev && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# Install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update && apt-get install -y yarn
# TODO: clean up

COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./static /app/static

RUN mkdir -p /app/src && touch /app/src/lib.rs
WORKDIR /app
RUN cargo build --release

# Build frontend
COPY ./front /app/front
RUN cd front && \
  yarn install && \
  yarn build && \
  cd .. && \
  rm -rf /app/front

# Include default assets
RUN mkdir /app/tmp
COPY ./assets/IPAexfont00401 /app/tmp/IPAexfont00401

COPY ./src /app/src
RUN cargo build --release

CMD cargo run --release
