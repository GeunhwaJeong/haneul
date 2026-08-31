# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# This tests the error message when you set your local client to an ephemeral network and then do `haneul client publish`

echo "== should fail and suggest test-publish or adding env to manifest =="
haneul client --client.config client.yaml publish
