#!/usr/bin/env bash
# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# Test an ephemeral publication workflow. We have
# B --> A
# C --> B
# C --> A
#
# D --> B
# D --> A
#
# E --> C
# E --> D
#
# We publish A, B, C, D, E in order

echo "=== expect to fail when publishing e because prereqs aren't published ==="
haneul client --client.config $CONFIG \
  test-publish --build-env testnet --pubfile-path Pub.local.toml e \
  > output.log 2>&1 || cat output.log


# publish a
echo ""
echo "=== this should be succesful ==="
haneul client --client.config $CONFIG \
  test-publish --build-env testnet --pubfile-path Pub.local.toml a \
  > output.log 2>&1 || cat output.log

# trying to republish should fail now.
echo ""
echo "=== this should fail ==="
haneul client --client.config $CONFIG test-publish --build-env testnet --pubfile-path Pub.local.toml a
