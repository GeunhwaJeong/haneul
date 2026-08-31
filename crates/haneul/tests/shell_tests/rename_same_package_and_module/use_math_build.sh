# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# tests that building a package that uses two packages that both define their name as "math" works.
haneul move --client.config $CONFIG build -p use_math
