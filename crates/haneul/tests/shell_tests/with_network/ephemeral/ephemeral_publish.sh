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

for i in a b c d e
do
  echo === building $i ===
  haneul client --client.config $CONFIG \
    test-publish --build-env testnet --pubfile-path Pub.local.toml $i \
    > out.log 2>&1 || cat out.log

done
