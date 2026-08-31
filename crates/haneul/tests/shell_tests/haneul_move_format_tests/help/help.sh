# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# `--help` is forwarded to `prettier-move` (rather than intercepted by clap)
# so the user sees prettier-move's full help.

chmod +x stubs/prettier-move
export PATH="$PWD/stubs:$(dirname "$(command -v haneul)")"

haneul move --client.config $CONFIG format --help
