# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# tests that haneul move new followed by haneul move build succeeds

haneul move --client.config $CONFIG new example
cd example && haneul move --client.config $CONFIG build
