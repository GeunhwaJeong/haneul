# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# This should succeed - the manifest has a broken dep, but the lockfile has it pinned to the correct location
# Repinning shouldn't be retriggered just from an added comment
echo '# comment' >> Move.toml
haneul move --client.config $CONFIG build
