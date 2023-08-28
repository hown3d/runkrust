FROM rust:1.63.0-slim-buster as chef
WORKDIR /workspace
RUN cargo install cargo-chef --locked

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
WORKDIR /workspace
COPY --from=planner /workspace/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

FROM golang:1.18

WORKDIR /workspace
RUN apt-get update && apt-get install -y nodejs npm 
RUN npm install -g tap

COPY runtime-tools/ .
RUN \
  --mount=type=cache,target=/root/.cache/go-build \
  --mount=type=cache,target=/go/pkg \
  make EXTRA_FLAGS="-buildvcs=false" runtimetest validation-executables

COPY --from=builder /workspace/target/release/runkrust /bin/

ENV RUNTIME=runkrust
# ENV RUST_BACKTRACE=full

ENTRYPOINT [ "make", "localvalidation" ]