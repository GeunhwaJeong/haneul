# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# If the client config file already exists, it doesn't change
cp client-exists.yaml before.yaml
haneul move --client.config ./client-exists.yaml -y new example

echo "diffing before/after; there should be no change"
diff client-exists.yaml before.yaml
