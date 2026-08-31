# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# If the config file doesn't exist, we prompt and then create it
# This is the same as skip.sh except we write <enter> on stdin and don't pass -y
echo "" | haneul move --client.config ./client.yaml new example \
  | sed 's/Generated new keypair.*$/Generated new keypair <REDACTED>/g' \
  | sed 's/recovery phrase : \[.*\]/recovery phrase : <REDACTED>/g'

sed 's/active_address:.*$/active_address: <REDACTED>/g' client.yaml

echo ""
echo "check for keystore:"
ls haneul.keystore
