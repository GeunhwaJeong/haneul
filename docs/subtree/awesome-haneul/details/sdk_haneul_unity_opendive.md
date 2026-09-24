# Haneul Unity SDK (OpenDive)

## Tooling Category

- [ ] dApp Development
- [ ] Explorer
- [ ] IDE
- [ ] Indexer
- [ ] Oracle
- [x] SDK

## Description

The OpenDive Haneul Unity SDK is the first fully-featured Unity SDK with offline transaction building.

This means that games built with our SDK can directly craft custom Move calls without relying Haneul's "unsafe" RPC calls under the [Transaction Builder API](https://docs.haneul.io/haneul-api-ref#transaction-builder-api) -- which in turn reduces the number of RPC / Network requests.

## Features

- [Features](https://github.com/OpenDive/Haneul-Unity-SDK?tab=readme-ov-file#features)
- ⚠️ `Bech32` encoded private key is not supported.
- ⚠️ GraphQL is not supported.
- Haneul BCS types are supported