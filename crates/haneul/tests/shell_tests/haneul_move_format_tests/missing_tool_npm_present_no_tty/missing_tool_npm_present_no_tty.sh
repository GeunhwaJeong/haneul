# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# `prettier-move` missing, `npm` present (stubbed), stdin is not a TTY (the
# harness pipes stdin from cargo nextest). `haneul move format` must skip the
# interactive prompt and bail with the manual-install hint. The stub `npm`
# only handles `--version`; if the install path is reached by mistake, the
# stub fails loudly.

chmod +x stubs/npm
export PATH="$PWD/stubs:$(dirname "$(command -v haneul)")"

haneul move --client.config $CONFIG format -c
