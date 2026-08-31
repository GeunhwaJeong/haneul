# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# Test to make sure we properly randomize addresses across packages.
haneul move --client.config $CONFIG summary --path data/overlapping_summaries
ls -1 data/overlapping_summaries/package_summaries/
cat data/overlapping_summaries/package_summaries/child_pkg/a.json
cat data/overlapping_summaries/package_summaries/other_child/a.json
cat data/overlapping_summaries/package_summaries/overlapping_summaries/b.json
cat data/overlapping_summaries/package_summaries/address_mapping.json
