ARG COMPONENT

FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

# Planning install
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Building all targets
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Base os
FROM alpine AS runtime-base
RUN addgroup -S nova && adduser -S nova -G nova
RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*

# Final os
FROM runtime-base AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${COMPONENT} /usr/local/bin/
USER nova
ENV COMPONENT ${COMPONENT}
CMD /usr/local/bin/${COMPONENT}
