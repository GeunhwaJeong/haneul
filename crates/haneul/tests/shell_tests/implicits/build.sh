# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# tests that building a package that implicitly depends on `haneul` can build
haneul move --client.config $CONFIG build -p example 2> /dev/null
