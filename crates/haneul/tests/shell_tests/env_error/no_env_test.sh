# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# This tests the error message when you set your local client to an ephemeral network and then do `haneul move test`

echo "== should fail and ask user to provide -e =="
haneul move --client.config client.yaml test
