# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# tests also work for the same scenario (one package uses two packages that name themselves `math`)
haneul move --client.config $CONFIG test -p use_math
