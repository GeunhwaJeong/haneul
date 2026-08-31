# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

haneul client --client.config config.yaml switch --env base

haneul client --client.config config.yaml envs
haneul client --client.config config.yaml --client.env one envs
haneul client --client.config config.yaml --client.env two envs

haneul client --client.config config.yaml active-env
haneul client --client.config config.yaml --client.env one active-env
haneul client --client.config config.yaml --client.env two active-env

# Unknown name -- Should give you None and nothing active
haneul client --client.config config.yaml --client.env not_an_env envs
haneul client --client.config config.yaml --client.env not_an_env active-env
