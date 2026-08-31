# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# This should fail - the manifest has a broken dep, and although the lockfile
# has it pinned to the correct location, we've edited the manifest so it should cause repinning
echo 'another_dep = { local = "another_dep" }' >> Move.toml
haneul move --client.config $CONFIG build
