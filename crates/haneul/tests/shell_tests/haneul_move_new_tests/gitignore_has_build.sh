# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# haneul move new example when `example/.gitignore` already contains build/*; it should be unchanged
mkdir example
echo "ignore1" >> example/.gitignore
echo "build/*" >> example/.gitignore
echo "ignore2" >> example/.gitignore
haneul move --client.config $CONFIG new example
cat example/.gitignore
echo
echo ==== files in example/ ====
ls -A example
