# Build application
#
# Single-stage builder + minimal runtime. Layer cache short-circuits when the
# COPY inputs are byte-identical to a previous build.
#
FROM rust:1.96.1-bullseye AS builder
RUN apt-get update \
 && apt-get install -y --no-install-recommends cmake clang libpq5 libpq-dev \
 && rm -rf /var/lib/apt/lists/*
ARG PROFILE=release
WORKDIR /haneul

# COPYs ordered least- to most-frequently-changing for deeper layer cache hits.
COPY rust-toolchain.toml ./
COPY .cargo .cargo
COPY Cargo.toml Cargo.lock ./
COPY external-crates external-crates
COPY haneul-execution haneul-execution
COPY consensus consensus
COPY crates crates

# GIT_REVISION declared late so SHA changes don't invalidate the COPYs above.
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN cargo build --locked --profile ${PROFILE} \
    --bin haneul-faucet \
    --bin haneul-bridge \
    --bin bridge-indexer \
    --bin haneul-bridge-cli \
    --bin haneul \
    --bin haneul-cluster-test \
    --bin haneul-tool \
    --bin move-analyzer \
    --bin haneul-metric-checker

# Production Image
FROM debian:bullseye-slim AS runtime
# haneul-tool needs libpq at runtime and git when fetching Move dependencies.
# openssh-client is explicit — without it `--no-install-recommends` drops the
# git→ssh-client recommendation, breaking git@/ssh:// Move sources.
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates git openssh-client libpq5 libpq-dev \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /haneul
# Source path is hardcoded to /release/ because the bench profile inherits
# release's dir-name.
COPY --from=builder /haneul/target/release/haneul-faucet       /usr/local/bin/haneul-faucet
COPY --from=builder /haneul/target/release/haneul-bridge       /usr/local/bin/haneul-bridge
COPY --from=builder /haneul/target/release/bridge-indexer   /usr/local/bin/bridge-indexer
COPY --from=builder /haneul/target/release/haneul-bridge-cli   /usr/local/bin/haneul-bridge-cli
COPY --from=builder /haneul/target/release/haneul              /usr/local/bin/haneul
COPY --from=builder /haneul/target/release/haneul-cluster-test /usr/local/bin/haneul-cluster-test
COPY --from=builder /haneul/target/release/haneul-tool         /usr/local/bin/haneul-tool
COPY --from=builder /haneul/target/release/move-analyzer    /usr/local/bin/move-analyzer
COPY --from=builder /haneul/target/release/haneul-metric-checker  /usr/local/bin/haneul-metric-checker

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
