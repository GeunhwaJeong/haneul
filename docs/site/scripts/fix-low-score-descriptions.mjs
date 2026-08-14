/*
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
*/

/**
 * Applies content-grounded rewrites to the goal.description strings that the
 * quality eval scored <= 2 (broken grammar, truncation, or scope mismatch).
 * Descriptions are hand-written from each page's actual headings and intro.
 *
 * Usage:
 *   node scripts/fix-low-score-descriptions.mjs          # dry run
 *   node scripts/fix-low-score-descriptions.mjs --apply  # write changes
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import matter from 'gray-matter';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const CONTENT_ROOT = path.resolve(__dirname, '..', '..', 'content');
const dryRun = !process.argv.includes('--apply');

const FIXES = {
  'develop/objects/versioning.mdx':
    'Reader understands how Haneul versions objects by (ID, version) and the fastpath versus consensus versioning paths',
  'develop/haneul-architecture/protocol-upgrades.mdx':
    'Reader understands how Haneul ships protocol and framework upgrades that validators adopt in lockstep',
  'develop/transactions/transaction-auth/auth-overview.mdx':
    'Reader understands cryptographic keys, addresses, and signatures on Haneul',
  'onchain-finance/asset-custody/address-balances/using-address-balances.mdx':
    'Reader can send, withdraw, pay gas from, and query address balances using the TypeScript SDK, CLI, and Move',
  'onchain-finance/payment-kit.mdx':
    'Reader understands the Payment Kit standard for secure payment processing with registries, receipts, and duplicate prevention',
  'develop/accessing-data/custom-indexer/custom-indexers.mdx':
    'Reader understands what custom indexers are, when to use them, and how the haneul-indexer-alt-framework ingestion, processing, and storage layers fit together',
  'develop/accessing-data/using-events.mdx':
    'Reader can define, emit, and query Move events to track onchain activity from offchain applications',
  'develop/publish-upgrade-packages/deploy.mdx':
    'Reader can compile and publish a Move package to a Haneul network',
  'develop/haneul-architecture/checkpoint-verification.mdx':
    'Reader can verify checkpoints and understands checkpoint commitments',
  'develop/haneul-architecture/haneul-security.mdx':
    "Reader understands Haneul's security guarantees for asset owners, from ownership and finality to auditing and censorship resistance",
  'develop/transactions/ptbs/building-ptb.mdx':
    'Reader can build programmable transaction blocks with the TypeScript SDK and CLI, including gas configuration and offline building',
  'develop/transactions/transaction-lifecycle.mdx':
    "Reader understands each stage of a transaction's lifecycle on Haneul, from creation through consensus, finality, and checkpoints",
  'onchain-finance/closed-loop-token/action-request.mdx':
    'Reader understands how an ActionRequest authorizes protected token actions and how to confirm one',
  'onchain-finance/deepbook-margin/contract-information/risk-ratio.mdx':
    'Reader understands how risk ratios determine leverage limits and collateral requirements in DeepBook Margin',
  'onchain-finance/deepbook-margin/margin-risks.mdx':
    'Reader understands the risks of margin trading on DeepBook, including liquidation and interest rate fluctuations',
  'onchain-finance/deepbookv3/contract-information/query-the-pool.mdx':
    'Reader can query pool state such as orders, balances, and quantities via the DeepBookV3 pool read API',
  'onchain-finance/examples-patterns/kiosk.mdx':
    'Reader can use the Kiosk standard to join tokenized assets while enforcing transfer policies',
  'onchain-finance/fungible-tokens/integrating-with-stablecoins.mdx':
    'Reader learns what stablecoins are and where they are used on Haneul',
  'onchain-finance/fungible-tokens/haneul-bridging.mdx':
    'Reader can bridge tokens to and from Haneul using Haneul Bridge and Wormhole, and understands their limits and supported assets',
  'onchain-finance/kiosk/kiosk-example.mdx':
    'Reader can open and configure a Haneul Kiosk and understands its guarantees for owners, buyers, marketplaces, and creators',
  'onchain-finance/payments.mdx':
    'Reader can integrate payment flows on Haneul, from reading and managing balances to sponsoring gasless transactions',
  'operators/data-management/managing-data.mdx':
    'Operator understands data management on Haneul full nodes and can configure pruning and archival policies to optimize their node',
  'references/contribute/contribute-to-haneul-repos.mdx':
    'Contributor can find how to open issues, fork, and submit PRs and SIPs to Haneul repositories',
  'references/contribute/contribution-process.mdx':
    'Contributor can edit Haneul docs via the GitHub web editor or a local environment and understands the review process',
  'references/contribute/localize-haneul-docs.mdx':
    'Contributor learns that Haneul docs are localized through Crowdin',
  'references/contribute/mdx-components.mdx':
    'Contributor can use the custom MDX components available in Haneul docs, such as tabs, admonitions, and ImportContent',
  'references/gaming.mdx':
    'Game developer can learn how to use Haneul features such as dynamic NFTs, Kiosk, soulbound assets, and onchain randomness to build games',
  'references/ptb-commands.mdx':
    "Reader can look up each PTB command's form, return type, and signature",
  'references/haneul-api/rpc-best-practices.mdx':
    'Reader can apply RPC best practices when configuring RPC provider settings',
  'references/haneul-move.mdx':
    'Reader can find links to Move language references, the Move Book, and Haneul framework docs',
  'references/ts-asset-tokenization.mdx':
    'Reader can look up how the tokenized_asset module represents real-world assets as onchain fractional tokens',
  'haneul-stack/haneulplay0x1/best-practices.mdx':
    'Reader can apply best practices for transaction handling, gas, and data storage when developing for HaneulPlay0X1',
  'haneul-stack/haneulplay0x1/migration-strategies.mdx':
    'Reader can support wallet and asset migration flows between on-device and off-device play in the Haneul gaming ecosystem',
  'haneul-stack/walrus/indexer-walrus.mdx':
    'Reader can build a custom indexer for a blog platform backed by Walrus content-addressable storage',
};

let applied = 0;
let missing = 0;

for (const [relPath, newDesc] of Object.entries(FIXES)) {
  const filePath = path.join(CONTENT_ROOT, relPath);
  if (!fs.existsSync(filePath)) {
    console.error(`WARNING: not found: ${relPath}`);
    missing++;
    continue;
  }
  const raw = fs.readFileSync(filePath, 'utf8');
  const { data, content: body } = matter(raw);
  if (!data.goal) {
    console.error(`WARNING: no goal on ${relPath}`);
    continue;
  }
  const oldDesc = data.goal.description;
  if (oldDesc === newDesc) continue;

  if (dryRun) {
    console.log(relPath);
    console.log(`  OLD: ${oldDesc}`);
    console.log(`  NEW: ${newDesc}\n`);
  } else {
    data.goal.description = newDesc;
    fs.writeFileSync(filePath, matter.stringify(body, data), 'utf8');
  }
  applied++;
}

console.log(`${'─'.repeat(50)}`);
console.log(`${dryRun ? 'DRY RUN' : 'APPLIED'}  fixed=${applied} missing=${missing}`);
if (dryRun) console.log('Run with --apply to write changes.');
