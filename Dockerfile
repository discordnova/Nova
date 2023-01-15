FROM rust AS chef
USER root
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y protobuf-compiler
WORKDIR /app

# Planning install
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Building all targets
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release 

# Base os
FROM debian:latest AS runtime-base
# RUN addgroup -S nova && adduser -S nova -G nova
RUN apt-get update && apt-get install ca-certificates -y

# Final os
FROM runtime-base AS runtime
ARG COMPONENT
ENV COMPONENT=${COMPONENT}
COPY --from=builder /app/target/release/${COMPONENT} /usr/local/bin/
# USER nova
ENTRYPOINT /usr/local/bin/${COMPONENT}
