// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import React from "react";
import Layout from "@theme/Layout";
import API from "../components/API";

import useDocusaurusContext from "@docusaurus/useDocusaurusContext";

export default function JsonRpc() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout title={`Haneul API Reference | ${siteConfig.title}`}>
      <div style={{ maxWidth: '960px', margin: '0 auto', padding: '1rem' }}>
        <p>Complete reference for the Haneul JSON-RPC API. Browse available methods, request parameters, and response schemas for interacting with the Haneul network programmatically.</p>
      </div>
      <API />
    </Layout>
  );
}
