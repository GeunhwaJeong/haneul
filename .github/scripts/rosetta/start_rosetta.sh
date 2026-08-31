#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

echo "Start Rosetta online server"
haneul-rosetta start-online-server --data-path ./data &

echo "Start Rosetta offline server"
haneul-rosetta start-offline-server &
