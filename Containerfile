FROM rust:latest

RUN apt update -y
RUN apt install -y nodejs npm clang libc++-dev libclang-dev libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

RUN rustup target add wasm32-unknown-unknown

RUN cargo install dioxus-cli --version 0.6.0-alpha.4
RUN rustup component add rustfmt