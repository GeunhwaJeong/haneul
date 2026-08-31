# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# explicitly passed environment should be used, irrespective of the active env
echo 'duplicate_env = "1234"' >> Move.toml
haneul move --client.config configs/name_match_id_mismatch.yaml build -e manifest_env
