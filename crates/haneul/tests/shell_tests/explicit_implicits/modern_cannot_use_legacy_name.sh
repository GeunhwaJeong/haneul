# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# tests that building a package that has explicit deps on legacy system names errors
haneul move --client.config $CONFIG build -p modern_using_legacy_name
