#!/usr/bin/env bash
# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# Publishing a package and then doing a test-publish succeed

chain_id=$(haneul client --client.config $CONFIG chain-identifier --format=hex)

echo "[environments]" >> a/Move.toml
echo "localnet = \"$chain_id\"" >> a/Move.toml

echo "=== real publish ==="
haneul client --client.config $CONFIG publish a \
  > out.log 2>&1 || cat out.log

echo "=== ephemeral publish ==="
haneul client --client.config $CONFIG \
  test-publish --build-env localnet --pubfile-path Pub.local.toml a \
  > out.log 2>&1 || cat out.log
