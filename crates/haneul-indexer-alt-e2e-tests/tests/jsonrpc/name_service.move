// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --simulator

// Testing various input errors for the HaneulNS name resolution:
// 1. Not enough labels (need at least two)
// 2. Too long
// 3. Bad (inconsistent) use of separators
// 4. Indvidual label too short
// 5. Individual label too long
// 6, 7, 8, 9. Bad characters (non-alphanumeric, or leading/trailing hyphen)

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["foo"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.haneul"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["foo*bar.haneul"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["foo..haneul"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["toolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolong.haneul"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["-foo.haneul"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["foo-.haneul"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["foo_bar.haneul"]
}

//# run-jsonrpc
{
  "method": "haneulx_resolveNameServiceAddress",
  "params": ["🫠.haneul"]
}
