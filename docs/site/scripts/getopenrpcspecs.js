// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

const axios = require('axios');
const fs = require('fs');
const path = require('path');

// Create directory
const topdir = path.join(__dirname, "../src/open-spec");

if (!fs.existsSync(topdir)){
    fs.mkdirSync(topdir);
}

// The repo has a single main branch; every network serves the same spec.
const downloadFile = async (network, branch = "main") => {
  const branchDir = path.join(topdir, network);
  const specDir = path.join(__dirname, `../src/open-spec/${network}`);
  const specFile = path.join(specDir, "openrpc.json");
  const backupFile = path.join(specDir, "openrpc_backup.json");

  if (!fs.existsSync(branchDir)) {
    fs.mkdirSync(branchDir, { recursive: true });
  }

  if (!fs.existsSync(specDir)) {
    fs.mkdirSync(specDir, { recursive: true });
  }

  try {
    const res = await axios.get(
      `https://raw.githubusercontent.com/GeunhwaJeong/haneul/${branch}/crates/haneul-open-rpc/spec/openrpc.json`
    );

    if (fs.existsSync(backupFile)) {
      fs.unlinkSync(backupFile);
      console.log(`Deleted ${network} backup spec.`);
    }

    if (fs.existsSync(specFile)) {
      fs.renameSync(specFile, backupFile);
      console.log(`Moved ${network} spec to backup.`);
    }

    fs.writeFileSync(specFile, JSON.stringify(res.data, null, 2), "utf8");
    console.log(`Downloaded ${network} spec.`);
  } catch (err) {
    console.error(`Error downloading ${network} openrpc spec.`, err.message);
  }
};

// Download Mainnet OpenRPC spec
downloadFile("mainnet");

// Download Testnet OpenRPC spec
downloadFile("testnet");

// Download Devnet OpenRPC spec
downloadFile("devnet");
