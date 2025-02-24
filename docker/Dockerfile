##
# builder image
##
FROM rust:1.83-alpine AS builder

ARG REPO_DIR=.

RUN apk --no-cache add \
  musl-dev \
  protobuf-dev \
  g++ \
  clang15-dev \
  linux-headers \
  wasm-pack \
  openssl-dev

WORKDIR /work

COPY "$REPO_DIR" .

ENV RUSTFLAGS="-C target-feature=-crt-static" \
  CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse"

RUN cargo build --release


##
# runtime image
##
FROM alpine

ARG VERSION
ENV VERSION=$VERSION

RUN apk --no-cache add libstdc++

COPY --from=builder /work/target/release/kcheck /usr/local/bin/

USER 13337:13337

ENTRYPOINT ["kcheck"]
CMD ["--help"]
