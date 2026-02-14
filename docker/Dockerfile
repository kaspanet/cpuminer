##
# builder image
##
FROM rust:1.88-alpine AS builder

ARG REPO_DIR=.

RUN apk --no-cache add \
  musl-dev \
  g++ \
  clang15-dev \
  linux-headers \
  protoc

WORKDIR /work

ENV RUSTFLAGS="-C target-feature=-crt-static -C link-args=-s" \
  CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse"

COPY "$REPO_DIR" .
RUN cargo build --release


##
# runtime image
##
FROM alpine

RUN apk --no-cache add \
  libstdc++

COPY --from=builder /work/target/release/kaspa-miner /usr/local/bin/

USER 13337:13337

ARG VERSION
ENV VERSION=$VERSION

ENTRYPOINT ["kaspa-miner"]
