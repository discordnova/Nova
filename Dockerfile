# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM tonistiigi/xx:master AS xx
FROM --platform=$BUILDPLATFORM rust:alpine as rbuild
RUN apk add clang lld protobuf protobuf-dev git build-base mingw-w64-gcc
COPY . .
COPY --from=xx / /

ARG TARGETPLATFORM
RUN xx-cargo build --release --target-dir ./build

FROM --platform=$BUILDPLATFORM alpine as passwd
RUN addgroup -S nova && adduser -S nova -G nova

FROM --platform=$BUILDPLATFORM golang:alpine as gbuild
RUN apk add clang lld
COPY --from=xx / /
ARG TARGETPLATFORM
COPY --from=rbuild /build/release/liball_in_one.a ./build/lib/liball_in_one.a
COPY . .
RUN go build -a -ldflags '-s' -o build/bin/nova cmd/nova/nova.go


FROM scratch as component
COPY --from=passwd /etc/passwd /etc/passwd
ARG COMPONENT
ENV COMPONENT=${COMPONENT}
COPY --from=rbuild /build/release/${COMPONENT} /usr/local/bin/
USER nova
ENTRYPOINT /usr/local/bin/${COMPONENT}

FROM scratch as all_in_one
