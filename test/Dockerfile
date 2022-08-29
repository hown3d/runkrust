FROM rust:1.63.0-slim-buster as builder
WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN \
  --mount=type=cache,target=/workspace/target/deps \
  --mount=type=cache,target=/workspace/target/build \
  --mount=type=cache,target=/workspace/target/incremental \
  cargo build

FROM golang:1.18

WORKDIR /workspace
RUN apt-get update && apt-get install -y nodejs npm 
RUN npm install -g tap

COPY runtime-tools/ .
RUN \
  --mount=type=cache,target=/root/.cache/go-build \
  --mount=type=cache,target=/go/pkg \
  make EXTRA_FLAGS="-buildvcs=false" runtimetest validation-executables

COPY --from=builder /workspace/target/debug/runkrust /bin/

ENV RUNTIME=runkrust

ENTRYPOINT [ "make", "localvalidation" ]