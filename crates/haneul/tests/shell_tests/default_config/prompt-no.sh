# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# If the config file doesn't exist, we prompt and bail if the user says no
echo "nope" | haneul move --client.config ./client.yaml new example
cat client.yaml
cat haneul.keystore
