# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# check that haneul move new followed by haneul move test succeeds
haneul move --client.config $CONFIG new example
cd example && haneul move --client.config $CONFIG test
