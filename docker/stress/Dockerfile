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

# stress binary doesn't use GIT_REVISION at compile time, so we deliberately
# don't declare ARG/ENV GIT_REVISION in the builder — keeps the cargo-build
# layer cache key stable across commits when source content is unchanged.
RUN cargo build --locked --profile ${PROFILE} --bin stress

# Production Image
#
# Mirrors the docker/haneul-node and docker/haneul-tools pattern: slim base + only
# the runtime tools consumers actually invoke. Previously this stage used
# `rust:1.92-bullseye` plus a `git clone` of the full repo, producing a
# ~1.2 GB compressed image (vs haneul-node's ~114 MB). The oversized export
# made BuildKit's `#28 exporting to client directory` step prone to gRPC
# transport hangs under PVC/network contention.
FROM debian:bullseye-slim AS runtime
# Runtime needs:
#   - haneul-network-aux mounts its own entry.sh and runs `aws s3 cp` to fetch
#     genesis + keystore, with a `curl` fallback.
#   - haneul-antithesis uses the in-image entrypoint below (curl + bash only).
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl awscli \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /haneul
ARG PROFILE=release
COPY --from=builder /haneul/target/${PROFILE}/stress /usr/local/bin/stress
COPY docker/stress/entrypoint.sh /docker/stress/entrypoint.sh
RUN chmod +x /docker/stress/entrypoint.sh

# haneul-benchmark workloads (composite/adversarial/party/slow/max_package) read
# their Move source packages at runtime via BENCHMARK_MOVE_BASE_DIR (or the
# env!("CARGO_MANIFEST_DIR") fallback in crates/haneul-benchmark/src/workloads/
# mod.rs). The previous runtime `git clone` made these visible incidentally;
# now we ship only what those workloads actually publish. Each workload's
# Move.toml has `Haneul = { local = "../../../../../haneul-framework/packages/
# haneul-framework" }`, which (relative to the Move.toml's parent dir) resolves
# to /haneul/crates/haneul-framework/packages/haneul-framework — so that tree plus its
# move-stdlib dep must also be present. Total cost: ~2 MB.
ENV BENCHMARK_MOVE_BASE_DIR=/haneul/crates/haneul-benchmark
COPY crates/haneul-benchmark/src/workloads/data /haneul/crates/haneul-benchmark/src/workloads/data
COPY crates/haneul-framework/packages/haneul-framework /haneul/crates/haneul-framework/packages/haneul-framework
COPY crates/haneul-framework/packages/move-stdlib /haneul/crates/haneul-framework/packages/move-stdlib

# publish_basics_package callers (shared_counter etc.) read examples/move/basics at runtime.
ENV MOVE_EXAMPLES_DIR=/haneul/examples/move
COPY examples/move/basics /haneul/examples/move/basics

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

CMD ["/docker/stress/entrypoint.sh"]
