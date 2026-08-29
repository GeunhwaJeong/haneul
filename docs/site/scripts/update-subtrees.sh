#!/usr/bin/env bash

# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

cd "$(git rev-parse --show-toplevel)" || exit 1

git subtree pull --prefix=docs/site/src/shared git@github.com:GeunhwaJeong/ML-Shared-Docusaurus.git master --squash
git subtree pull --prefix=docs/subtree/awesome-haneul git@github.com:haneul-foundation/awesome-haneul.git main --squash
git subtree pull --prefix=docs/subtree/awesome-gaming git@github.com:becky-haneul/awesome-haneul-gaming.git main --squash

echo "✅ All subtree content updated — commit and push the changes"
