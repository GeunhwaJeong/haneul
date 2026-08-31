# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0

# tests that haneul move new followed by haneul move disassemble succeeds


haneul move --client.config $CONFIG new example
cat > example/sources/example.move <<EOF
module example::example;

public fun foo(_ctx: &mut TxContext) {}
EOF
cd example

echo "=== Build ===" >&2
haneul move --client.config $CONFIG build

echo "=== Disassemble ===" >&2
haneul move --client.config $CONFIG disassemble build/example/bytecode_modules/example.mv
