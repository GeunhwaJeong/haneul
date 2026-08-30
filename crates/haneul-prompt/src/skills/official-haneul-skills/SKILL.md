---
name: official-haneul-skills
description: >
  Pointer to the official Haneul Labs skills for building on Haneul — language fundamentals,
  object model, PTBs, SDKs, publishing, upgrades, frontend integration, accessing on-chain
  data. Maintained upstream at github.com/GeunhwaJeong/skills; pinned to the same ref the
  audit catalog derives from (see maintenance/UPSTREAMS.md). Trigger on "build a contract",
  "publish a package", "upgrade a module or package", "use the TypeScript SDK", "write a PTB",
  "set up a Haneul client".
---

# Official Haneul skills (upstream pointer)

For building, publishing, and upgrading Move contracts on Haneul — and the SDK / CLI /
frontend integration around them — refer to the official skills maintained by
Haneul Labs. This bundle is a pointer, not embedded content.

- Repository: <https://github.com/GeunhwaJeong/skills>
- Pinned snapshot (same upstream snapshot the audit catalog tracks):
  <https://github.com/GeunhwaJeong/skills/tree/764f21a95e709f46c60877a59d6ee6f27d9ed91e>

## High-level scope at the pinned ref

- `haneul-move/` — Move on Haneul: language fundamentals, events, coins
- `object-model/` — ownership, transfers, dynamic fields, display, patterns
- `ptbs/` — Programmable Transaction Blocks (fundamentals, building, troubleshooting, cli)
- `composable-move-functions/`, `naming-conventions/`, `modern-move-syntax/`,
  `move-unit-testing/`, `haneul-move-project/`, `haneul-build-test/`
- `haneul-publish/` — package publishing
- `haneul-cli/`, `haneul-client/`, `haneul-install/` — CLI / client / install
- `frontend-apps/` — TypeScript SDK integration
- `haneul-sdks/` — TypeScript and Rust SDKs
- `accessing-data/` — gRPC, GraphQL, indexers, Walrus, archival
- `haneul-overview/` — ecosystem framing

## Fetching individual files

Rendered (browser-friendly, HTML):

  <https://github.com/GeunhwaJeong/skills/blob/764f21a95e709f46c60877a59d6ee6f27d9ed91e/{skill}/{file}.md>

Raw (plain markdown, easier for programmatic consumption):

  <https://raw.githubusercontent.com/GeunhwaJeong/skills/764f21a95e709f46c60877a59d6ee6f27d9ed91e/{skill}/{file}.md>

Pick whichever your fetch tool handles best. Both serve the same content at the
same pinned snapshot.
