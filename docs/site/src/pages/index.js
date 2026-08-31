// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import React from "react";

import Layout from "@theme/Layout";
import Head from "@docusaurus/Head";
import Link from "@docusaurus/Link";
import styles from "./index.module.css";

// Target for the "Developer Updates" hero link. That page does not exist yet,
// so this points at the external Haneul blog as an interim (external links are not
// route-checked, so the strict build passes). Swap this single constant to
// "/developer-updates" when the dedicated page ships.
export const DEVELOPER_UPDATES_URL = "https://blog.haneul.io";

export default function Home() {
  const developerResources = [
    {
      title: "Getting Started",
      description:
        "Install the Haneul toolchain, set up a wallet, and publish your first Move package.",
      to: "/getting-started",
    },
    {
      title: "Haneul Agent Skills",
      description: "Equip AI coding agents with Haneul-specific skills and context.",
      to: "/skills",
    },
    {
      title: "Develop",
      description:
        "Write and upgrade Move packages, work with objects, and query onchain data.",
      to: "/develop",
    },
    {
      title: "Onchain Finance",
      description:
        "Issue tokens and stablecoins, tokenize assets, and build payments and NFTs.",
      to: "/onchain-finance",
    },
    {
      title: "Haneul Stack",
      description:
        "Compose onchain primitives like zkLogin, Nautilus, and Seal into your app.",
      to: "/haneul-stack",
    },
    {
      title: "References",
      description:
        "Look up the CLI, SDKs, Move framework, and network API references.",
      to: "/references",
    },
  ];

  const useCases = [
    {
      title: "DeepBook",
      description:
        "Trade on Haneul's onchain central limit order book across spot, margin, and prediction markets.",
      to: "/onchain-finance/deepbook",
    },
    {
      title: "Walrus",
      description:
        "Store and serve media, blobs, and app data on decentralized storage.",
      to: "/haneul-stack/walrus",
    },
    {
      title: "zkLogin",
      description:
        "Onboard users with their existing Web2 logins, no seed phrase required.",
      to: "/haneul-stack/zklogin-integration/zklogin",
    },
    {
      title: "Digital Assets",
      description:
        "Build NFTs and composable digital collectibles with the object model.",
      to: "/getting-started/examples/lootbox-ctf",
    },
  ];

  const nodeOperators = [
    {
      title: "Run a Haneul Full Node",
      description: "Run a full node to sync the network and serve onchain data.",
      to: "/operators/full-node/haneul-full-node",
    },
    {
      title: "Validators",
      description: "Set up and operate a validator to help secure the network.",
      to: "/operators/validator",
    },
    {
      title: "Data Management",
      description:
        "Set up archival storage and indexing services for onchain data.",
      to: "/operators/data-management",
    },
  ];

  const ResourceCard = ({ title, description, to }) => (
    <Link to={to} className={styles.resourceCard}>
      <h3 className={styles.resourceCardTitle}>{title}</h3>
      <p className={styles.resourceCardDesc}>{description}</p>
      <span className={styles.resourceCardArrow} aria-hidden="true">
        →
      </span>
    </Link>
  );

  const Section = ({ heading, items }) => (
    <section className={styles.homeSection}>
      <h2 className={styles.homeSectionHeading}>{heading}</h2>
      <div className="flex flex-row flex-wrap justify-center gap-2">
        {items.map((item) => (
          <ResourceCard key={item.title} {...item} />
        ))}
      </div>
    </section>
  );

  return (
    <>
      <Head>
        <meta
          name="google-site-verification"
          content="nOyG5Cxvr3m94VHwQFHHaK_5BR6EyAYJ_4oPxYBptPs"
        />
      </Head>
      <Layout>
        <div
          className="overflow-hidden min-h-screen flex flex-col bg-cover bg-center bg-no-repeat"
          style={{ backgroundColor: "#000000" }}
        >
          <div className="w-full mt-8 mb-4 mx-auto">
            <div className={styles.heroText}>
              <h1 className="h1 center-text text-white">Haneul Documentation</h1>
              <p
                className="center-text"
                style={{
                  color: "#89919F",
                  maxWidth: "720px",
                  margin: "0 auto",
                  fontSize: "1.1rem",
                  lineHeight: "1.6",
                }}
              >
                Haneul is a next-generation smart contract platform with high
                throughput, low latency, and an asset-oriented programming model
                powered by the Move programming language. Explore guides,
                references, and tutorials to start building on Haneul.
              </p>
              <Link to={DEVELOPER_UPDATES_URL} className={styles.devUpdates}>
                Developer Updates
              </Link>
            </div>
          </div>

          <Section heading="Developer Resources" items={developerResources} />
          <Section heading="Use Cases" items={useCases} />
          <Section heading="Node Operators" items={nodeOperators} />
        </div>
      </Layout>
    </>
  );
}
