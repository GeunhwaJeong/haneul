# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# If the config file is a directory, we fail nicely
mkdir client.yaml
haneul move --client.config ./client.yaml new example \
  | sed 's/Err:.*$/Err: <REDACTED>/'
