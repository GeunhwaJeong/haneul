#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0
#
# Check whether the version of framework in the repo is compatible
# with the version on chain, as reported by the currently active
# environment, using the binary in environment variable $HANEUL.

set -e

HANEUL=${HANEUL:-haneul}
REPO=$(git rev-parse --show-toplevel)

for PACKAGE in "$REPO"/crates/haneul-framework/packages/*; do
    $HANEUL client verify-source "$PACKAGE"
done

