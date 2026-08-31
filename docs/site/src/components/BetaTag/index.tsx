// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// This component is used to display beta tag.

import React from "react";
import Admonition from "@theme/Admonition";

export default function BetaTag(props) {
  if (!props.beta) {
    return;
  }

  const beta = props.beta.toLowerCase();
  // If `props.slim` is included, do not add spacing because the box is inline with
  // content as opposed to at top of topic based on `beta` frontmatter.
  return (
    <Admonition type="info"
      title="Early-Stage Feature"
      icon="⚙️"
      className={`${props.slim ? "" : "!my-12"} bg-haneul-blue-light border-haneul-blue-dark dark:bg-haneul-blue-dark dark:border-haneul-blue-light`}
    >
      <p className="pt-2">
        This content describes an alpha/beta feature or service. These early
        stage features and services are in active development, so details are
        likely to change.
      </p>
      {(beta.includes("testnet") ||
        beta.includes("devnet") ||
        beta.includes("mainnet")) && (
        <p>
          This feature or service is currently available in
          <ul className="mt-4">
            {beta.includes("devnet") && <li className="font-bold">Devnet</li>}
            {beta.includes("testnet") && <li className="font-bold">Testnet</li>}
            {beta.includes("mainnet") && <li className="font-bold">Mainnet</li>}
          </ul>
        </p>
      )}
    </Admonition>
  );
}
