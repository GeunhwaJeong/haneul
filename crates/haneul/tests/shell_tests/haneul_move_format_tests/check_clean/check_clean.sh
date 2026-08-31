# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# Happy path: stub `prettier-move` exits 0. Asserts the user's args are
# forwarded verbatim.

chmod +x stubs/prettier-move
export PATH="$PWD/stubs:$(dirname "$(command -v haneul)")"

haneul move --client.config $CONFIG format -c example
