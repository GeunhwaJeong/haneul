# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# explicitly passed environment should be used, and we should fail if they aren't in the manifest
haneul move --client.config configs/name_mismatch_id_mismatch.yaml build -e cli_arg_env
