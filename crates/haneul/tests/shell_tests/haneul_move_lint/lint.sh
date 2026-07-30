# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# Tests that `haneul move lint` enables Haneul mode and runs the Haneul-specific linters (here
# `self_transfer`), not just the generic Move linters. `COLOR_MODE=NONE` disables ANSI
# color codes in the compiler diagnostics so the snapshot is stable.
COLOR_MODE=NONE haneul move --client.config $CONFIG lint -p example
