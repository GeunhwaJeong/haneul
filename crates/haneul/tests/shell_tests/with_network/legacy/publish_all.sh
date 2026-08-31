#!/usr/bin/env bash
# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# We publish a legacy package that depends on another legacy package
#
# legacy_dep <-- legacy
#            <-- modern <-- legacy_depends_on_modern
#
# We have to use test-publish because you can't real-publish a legacy package on localnet

echo "=== publish legacy_dep ==="
haneul client --client.config $CONFIG \
    test-publish --build-env testnet --pubfile-path Pub.local.toml legacy_dep \
    2> output.log > output.log && echo "success" || cat output.log

echo "=== publish legacy ==="
haneul client --client.config $CONFIG \
    test-publish --build-env testnet --pubfile-path Pub.local.toml legacy \
    2> output.log > output.log && echo "success" || cat output.log

echo "=== publish modern ==="
haneul client --client.config $CONFIG \
    test-publish --build-env testnet --pubfile-path Pub.local.toml modern \
    2> output.log > output.log && echo "success" || cat output.log

echo "=== publish legacy_depends_on_modern ==="
echo "    Should fail because legacy packages are not allowed to depend on modern packages"
haneul client --client.config $CONFIG \
    test-publish --build-env testnet --pubfile-path Pub.local.toml legacy_depends_on_modern \
    2> output.log > output.log && echo "success" || cat output.log
