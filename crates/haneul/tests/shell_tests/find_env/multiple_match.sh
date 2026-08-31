# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# Active environment chain ID matches multiple envs in the manifest
echo 'duplicate_env = "1234"' >> Move.toml
haneul move --client.config configs/name_mismatch_id_match.yaml build
