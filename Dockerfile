# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM tonistiigi/xx:master AS xx
FROM --platform=$BUILDPLATFORM rust:alpine as alpine_rbuild
RUN apk add clang lld protobuf-dev build-base git
# Copy the xx scripts
COPY --from=xx / /
# Copy source code
COPY . .

RUN --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/registry/index \
    cargo fetch
ARG TARGETPLATFORM
RUN --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/registry/index \
    xx-cargo build --release --target-dir ./build

#Copy from the build/<target triple>/release folder to the out folder
RUN mkdir ./out && cp ./build/*/release/* ./out || true

FROM alpine AS runtime
ARG COMPONENT
ENV COMPONENT=${COMPONENT}
COPY --from=alpine_rbuild /out/${COMPONENT} /usr/local/bin/
ENTRYPOINT /usr/local/bin/${COMPONENT}
