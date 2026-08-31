// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

const { main: validateOpenRpcSpecs } = require("../../scripts/validate-openrpc.js");

module.exports = function openrpcValidatePlugin() {
  return {
    name: "openrpc-validate-plugin",
    async loadContent() {
      // Throwing here aborts `docusaurus build` with a clear stack trace + message
      validateOpenRpcSpecs();
      return null;
    },
  };
};
