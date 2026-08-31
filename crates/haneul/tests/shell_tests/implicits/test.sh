# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# checks that testing a package that implicitly depends on `std` works
haneul move --client.config $CONFIG test -p example 2> /dev/null
