# Awesome Haneul [![Awesome](https://awesome.re/badge.svg)](https://awesome.re)

<a href="https://haneul.io/"><img alt="Haneul logo" src="media/logo.svg" align="right" width="150" /></a>

> A curated list of _awesome_ developer tools and infrastructure projects within the Haneul ecosystem.

Haneul is the first blockchain built for internet scale, enabling fast, scalable, and low-latency transactions. It's programmable and composable, powered by the Move language, making it easy to build and integrate dApps. Haneul prioritizes developer experience and frictionless user interactions, designed to support next-gen decentralized applications with minimal complexity.

> ⚠️ This warning icon means that the tool may not be functioning correctly at the moment. Please check these tools carefully.

[**Submit your own developer tool here**](CONTRIBUTING.md)

## Contents

- [Move IDEs](#move-ides)
  - [Web IDEs](#web-ides)
  - [Desktop IDEs](#desktop-ides)
  - [IDE Utilities](#ide-utilities)
- [Client SDKs \& Libraries](#client-sdks--libraries)
  - [Client SDKs](#client-sdks)
  - [DeFi SDKs](#defi-sdks)
  - [Client Libraries](#client-libraries)
- [dApp Development](#dapp-development)
  - [dApp Toolkits](#dapp-toolkits)
  - [Smart Contract Toolkits](#smart-contract-toolkits)
- [Indexers \& Data Services](#indexers--data-services)
- [Explorers](#explorers)
- [Oracles](#oracles)
- [Security](#security)
- [AI](#ai)
- [Infrastructure as Code](#infrastructure-as-code)
- [Faucets](#faucets)

## Move IDEs

### Web IDEs

- BitsLab IDE - Online Move code editor that requires no configuration and supports Move code syntax highlighting. Beginner friendly and supports interacting with Haneul.
  - [Homepage](https://www.bitslab.xyz/bitslabide) - [IDE](https://ide.bitslab.xyz/) - [Tutorial](https://www.youtube.com/watch?v=-9-WkqQwtu8) - [Further Information](details/ide_bitslab.md)
- MoveStudio - Online IDE for Haneul smart contract development.
  - [Homepage](https://www.movestudio.dev/) - [GitHub](https://github.com/dantheman8300/move-studio) - [IDE](https://www.movestudio.dev/build) - [Further Information](details/ide_movestudio.md)
- ChainIDE - Move Cloud-Powered Development Platform.
  - [Homepage](https://chainide.com) - [Documentation](https://chainide.gitbook.io/chainide-english-1/ethereum-ide-1/9.-haneul-ide) - [IDE](https://chainide.com/s/haneul) - [Further Information](details/ide_chainide.md)
- ⚠️ WELLDONE Code - Remix IDE plugin supports non-EVM smart contract development including Haneul.
  - [Homepage](https://docs.welldonestudio.io/code) - [Documentation & Tutorial](https://docs.welldonestudio.io/code/deploy-and-run/haneul) - [Further Information](details/ide_welldone_code.md)


### Desktop IDEs

- VSCode Move by Haneul Labs - VSCode Extension supports Move on Haneul development with LSP features through Move Analyzer developed by Haneul Labs.
  - [GitHub](https://github.com/GeunhwaJeong/haneul/tree/main/external-crates/move/crates/move-analyzer) - [Documentation & Tutorial](https://marketplace.visualstudio.com/items?itemName=haneullabs.move) - [Further Information](details/ide_vscode_haneullabs_move_analyzer.md)
- VSCode Haneul Move Analyzer by MoveBit - Alternative VSCode extension developed by MoveBit.
  - [Homepage](https://movebit.xyz/analyzer) - [GitHub](https://github.com/movebit/haneul-move-analyzer) - [Documentation & Tutorial](https://marketplace.visualstudio.com/items?itemName=MoveBit.haneul-move-analyzer) - [Further Information](details/ide_vscode_movebit_haneul_move_analyzer.md)
- IntelliJ Haneul Move Language Plugin - IntelliJ-based plugin for Move on Haneul development.
  - [Homepage](https://plugins.jetbrains.com/plugin/23301-haneul-move-language) - [GitHub](https://github.com/movefuns/intellij-move)
- [Emacs move-mode](https://github.com/amnn/move-mode) - The move-mode package is an Emacs major-mode for editing smart contracts written in the Move programming language.
- [Move.vim](https://github.com/yanganto/move.vim) - Syntax highlighting that supports the Move 2024 edition.

### IDE Utilities

- [Prettier Move Plugin](https://github.com/GeunhwaJeong/haneul/tree/main/external-crates/move/crates/move-analyzer/prettier-plugin) - A Move language plugin for the Prettier code formatter.
- [Haneul Extension](https://github.com/zktx-io/haneul-extension) - The Haneul extension provides seamless support for compiling, deploying, and testing Haneul smart contracts directly within VS Code.
  - [Homepage](https://marketplace.visualstudio.com/items?itemName=zktxio.haneul-extension) - [Documentation](https://docs.zktx.io/vsce/haneul/)
- ⚠️ Haneul Simulator - VSCode Extension to streamline Haneul development workflow with intuitive UI.
  - [Homepage](https://marketplace.visualstudio.com/items?itemName=weminal-labs.haneul-simulator-vscode) - [GitHub](https://github.com/Weminal-labs/haneul-simulator-vscode) - [Demo](https://www.youtube.com/watch?v=BHRxeF_visM&pp=ygUMd2VtaW5hbCBsYWIg)
- [Tree Sitter Move](https://github.com/tzakian/tree-sitter-move) - Tree Sitter for Move.

## Client SDKs & Libraries

### Client SDKs

- Haneul TypeScript SDK (Haneul Labs) - TypeScript modular library of tools for interacting with the Haneul blockchain.
  - [GitHub](https://github.com/GeunhwaJeong/haneul/tree/main/sdk/typescript) - [Documentation](https://sdk.haneul-labs.com/typescript) - [Further Information](details/sdk_haneul_typescript.md)
- Haneul Kit(Scallop) - Toolkit for interacting with the Haneul network in TypeScript.
  - [GitHub](https://github.com/scallop-io/haneul-kit) - [Further Information](details/sdk_haneul_kit_scallop.md)
- Haneul Rust SDK (Haneul Labs) - Rust SDK to interact with Haneul blockchain.
  - [GitHub](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk) - [Documentation](https://haneullabs.github.io/haneul/haneul_sdk/index.html) - [Further Information](details/sdk_haneul_rust.md)
- Pyhaneul - Python SDK to interact with Haneul blockchain.
  - [GitHub](https://github.com/FrankC01/pyhaneul?tab=readme-ov-file) - [Documentation](https://pyhaneul.readthedocs.io/en/latest/index.html) - [Pypi](https://pypi.org/project/pyhaneul/) - [Discord](https://discord.gg/uCGYfY4Ph4) - [Further Information](details/sdk_pyhaneul.md)
- Haneul Go SDK (HaneulVision) - Golang SDK to interact with Haneul blockchain.
  - [GitHub](https://github.com/block-vision/haneul-go-sdk) - [API Documentation](https://pkg.go.dev/github.com/block-vision/haneul-go-sdk) - [Examples](https://github.com/block-vision/haneul-go-sdk?tab=readme-ov-file#examples) - [Further Information](details/sdk_haneul_go.md)
- Haneul Go SDK (Pattonkan) - Golang SDK to interact with Haneul blockchain. Support PTB and devInspect.
  - [Github](https://github.com/pattonkan/haneul-go) - [API Documentation](https://pkg.go.dev/github.com/pattonkan/haneul-go) - [Examples](https://github.com/pattonkan/haneul-go/tree/main/examples) - [Further Information](details/go-haneul.md)
- Haneul Dart SDK - Dart SDK to interact with Haneul blockchain.
  - [GitHub](https://github.com/mofalabs/haneul) - [API documentation](https://pub.dev/documentation/haneul/latest/) - [Further Information](details/sdk_haneul_dart.md)
- Haneul Kotlin SDK - Kotlin Multiplatform (KMP) SDK for integrating with the Haneul blockchain.
  - [GitHub](https://github.com/mcxross/khaneul) - [Documentation](https://haneulcookbook.com) - [Further Information](details/sdk_khaneul.md)
- HaneulKit (OpenDive) - Swift SDK natively designed to make developing for the Haneul blockchain easy.
  - [GitHub](https://github.com/opendive/haneulkit?tab=readme-ov-file) - [Further Information](details/sdk_haneulkit.md)
- Haneul Unity SDK (OpenDive) - The OpenDive Haneul Unity SDK is the first fully-featured Unity SDK with offline transaction building.
  - [GitHub](https://github.com/OpenDive/Haneul-Unity-SDK) - [Further Information](details/sdk_haneul_unity_opendive.md)
- Dubhe Client (Dubhe Engine) - Supports various platforms including browsers, Node.js, and game engine. It provides a simple interface to interact with your Haneul Move contracts.
  - [GitHub](https://github.com/0xobelisk/dubhe/tree/main/packages/haneul-client) - [Documentation](https://dubhe.obelisk.build/dubhe/haneul/client)

### DeFi SDKs
- [NAVI Protocol SDK](https://github.com/naviprotocol/navi-sdk) - The NAVI TypeScript SDK Client provides tools for interacting with the Haneul blockchain networks, designed for handling transactions, accounts, and smart contracts efficiently.
- [Bucket Protocol SDK](https://github.com/Bucket-Protocol/bucket-protocol-sdk) - The TypeScript SDK for interacting with Bucket Protocol.
- [Haneullend SDK](https://github.com/solendprotocol/haneullend-public/tree/production/sdk) - The TypeScript SDK for interacting with the Haneullend program published on npm as [`@haneullend/sdk`](https://www.npmjs.com/package/@haneullend/sdk).
- [Scallop SDK](https://github.com/scallop-io/haneul-scallop-sdk) - The TypeScript SDK for interacting with the Scallop lending protocol on the Haneul network.
- [Cetus CLMM SDK](https://github.com/CetusProtocol/cetus-clmm-haneul-sdk) - The official Cetus SDK specifically designed for seamless integration with Cetus-CLMM on Haneul.
- [Aftermath SDK](https://github.com/AftermathFinance/aftermath-ts-sdk) - The TypeScript SDK for interacting with Aftermath Protocol.
- [FlowX SDK](https://github.com/FlowX-Finance/sdk) - The official FlowX TypeScript SDK that allows developers to interact with FlowX protocols using the TypeScript programming language.
- [7k Aggregator SDK](https://github.com/7k-ag/7k-sdk-ts) - The TypeScript SDK for interacting with 7k Aggregator protocol.
- [Hop Aggregator SDK](https://docs.hop.ag/hop-sdk) - The TypeScript SDK for interacting with Hop Aggregator.

### Client Libraries

- [BCS TypeScript (Haneul Labs)](https://sdk.haneul-labs.com/bcs) - BCS with TypeScript.
- [BCS Rust](https://github.com/zefchain/bcs) - BCS with Rust.
- [BCS Dart](https://github.com/mofalabs/bcs) - BCS with Dart.
- BCS Kotlin - BCS with Kotlin.
  - [GitHub](https://github.com/mcxross/kotlinx-serialization-bcs) - [Documentation](https://haneulcookbook.com/bcs.html)
- [BCS Swift](https://github.com/OpenDive/HaneulKit/tree/main/Sources/HaneulKit/Utils/BCS) - BCS with Swift.
- [BCS Unity](https://github.com/OpenDive/Haneul-Unity-SDK/tree/main/Assets/Haneul-Unity-SDK/Code/OpenDive.BCS) - BCS with Unity C#.
- [Haneul Client Gen (Kuna Labs)](https://github.com/kunalabs-io/haneul-client-gen/tree/master) - A tool for generating TS SDKs for Haneul Move smart contracts. Supports code generation both for source code and on-chain packages with no IDLs or ABIs required.
- [TypeMove (Sentio)](https://github.com/sentioxyz/typemove/blob/main/packages/haneul/Readme.md) - Generate TypeScript bindings for Haneul contracts.
- Haneul Wallet Standard (Haneul Labs) - A suite of standard utilities for implementing wallets and libraries based on the [Wallet Standard](https://github.com/wallet-standard/wallet-standard/).
  - [GitHub](https://github.com/GeunhwaJeong/haneul/tree/main/sdk/wallet-standard) - [Documentation](https://docs.haneul.io/standards/wallet-standard)
- [CoinMeta (Polymedia)](https://github.com/juzybits/polymedia-coinmeta) - Library for fetching coin metadata for Haneul coins.
- [Dubhe Client BCS Decoding (Dubhe Engine)](https://github.com/0xobelisk/dubhe-docs/blob/main/pages/dubhe/haneul/client.mdx#bcs-data-decoding) - Library for supports automatic parsing of BCS types based on contract metadata information and automatic conversion formatting.

## dApp Development

### dApp Toolkits

- [@haneullabs/create-dapp](https://sdk.haneul-labs.com/dapp-kit/create-dapp) - CLI tool that helps you create Haneul dApp projects.
- Haneul dApp Kit (Haneul Labs) - Set of React components, hooks, and utilities to help you build a dApp for the Haneul ecosystem.
  - [GitHub](https://github.com/GeunhwaJeong/haneul/tree/main/sdk/dapp-kit) - [Documentation](https://sdk.haneul-labs.com/dapp-kit)
- Haneul dApp Starter - Full-stack boilerplate which lets you scaffold a solid foundation for your Haneul project and focus on the business logic of your dapp from day one.
  - [GitHub](https://github.com/haneulware/haneul-dapp-starter?tab=readme-ov-file) - [Documentation](https://haneul-dapp-starter.dev/docs/) - [Demo app](https://demo.haneul-dapp-starter.dev/)
- Haneulet Wallet Kit - React toolkit for aApps to interact with all wallet types in Haneul easily.
  - [GitHub](https://github.com/haneulet/wallet-kit) - [Documentation](https://kit.haneulet.app/docs/QuickStart)
- SmartKit - React library that allows your dapp to connect to the Haneul network in a simple way.
  - [Homepage](https://smartkit.vercel.app/) - [GitHub](https://github.com/heapup-tech/smartkit)
- [Haneul Haneultcase](https://github.com/juzybits/polymedia-haneultcase) - Haneul utilities for TypeScript, Node, and React.
- [Haneul MultiSig Toolkit (Haneul Labs)](https://multisig-toolkit.vercel.app/offline-signer) - Toolkit for transaction signing.
- [Haneul dApp Scaffold (Bucket Protocol)](https://github.com/Bucket-Protocol/haneul-dapp-scaffold-v1) - A frontend scaffold for a decentralized application (dApp) on the Haneul blockchain.
- [Wormhole Kit (zktx.io)](https://github.com/zktx-io/wormhole-kit-monorepo) - React library that enables instant integration of Wormhole into your dapp.
- HaneulBase - Haneulbase makes it easy to create "workdirs", each defining a distinct development environment targeting a network.
  - [GitHub](https://github.com/chainmovers/haneulbase) - [Documentation](https://haneulbase.io/)
- [create-dubhe (Dubhe Engine)](https://github.com/0xobelisk/dubhe/tree/main/packages/create-dubhe) - Create a new Dubhe project on Haneul.
  - [Documentation](https://dubhe.obelisk.build/dubhe/haneul/quick-start)
- [Haneul Tools](https://haneul-tools.vercel.app/ptb-generator) - Scaffolding TypeScript PTBs for any on-chain function you might want to invoke.
- [Enoki (Haneul Labs)](https://docs.enoki.haneul-labs.com/) - Make zkLogin and Sponsored Transactions more accessible.
- [Haneul Gas Pool (Haneul Labs)](https://github.com/GeunhwaJeong/haneul-gas-pool) - Service that powers sponsored transactions on Haneul at scale.
- [useHaneulZkLogin](https://github.com/pixelbrawlgames/use-haneul-zklogin) - React hook and functions for seamless zkLogin integration on Haneul.
- @haneulware/kit - Opinionated React components and hooks for Haneul dApps.
  - [Homepage](https://kit.haneulware.io/) - [Documentation](https://github.com/haneulware/kit/tree/main/packages/kit#readme) - [GitHub](https://github.com/haneulware/kit)
- React ZK Login Kit - Ready-to-use Component with Hook (sign-in + sign-transaction)
  - [GitHub](https://github.com/denyskozak/react-haneul-zk-login-kit) - [YouTube Guide](https://www.youtube.com/watch?v=2qnjmKg3ugY)

#### zkLogin

- [zkLogin Demo (Polymedia)](https://github.com/juzybits/polymedia-zklogin-demo)
- [Haneul zkLogin Demo by @jovicheng](https://github.com/jovicheng/haneul-zklogin-demo)
- [Haneul zkWallet Demo by @ronanyeah](https://github.com/ronanyeah/haneul-zk-wallet)
- [zkLogin Demo using use-haneul-zklogin by @pixelbrawlgames](https://pixelbrawlgames.github.io/use-haneul-zklogin/)
- [zkLogin Demo using react-zk-login-kit by @denyskozak](https://demo.react-haneul-zk-login.com)

#### Misc

- [`haneul-sniffer`](https://www.app.kriya.finance/haneul-sniffer/) - Checking security of Haneul tokens.
- RPC Tools (Polymedia) - A webapp that lets users find the fastest RPC for their location.
  - [GitHub](https://github.com/juzybits/polymedia-rpcs) - [Documentation](https://rpcs.polymedia.app/)
- [Polymedia Commando (Polymedia)](https://github.com/juzybits/polymedia-commando) - Haneul command line tools to help with Haneul airdrops (send coins to many addresses), gather data from different sources (Haneul RPCs, Indexer.xyz, Haneulscan), and more.
- [YubiHaneul (HaneulLabs)](https://github.com/GeunhwaJeong/yubigen) - Create a Haneul Wallet inside a yubikey and sign Haneul transactions with it.
- [`haneul-dapp-kit-theme-creator`](https://haneul-dapp-kit-theme-creator.app/) - Build custom Haneul dApp Kit themes.
- [Minting Server (Haneul Labs)](https://github.com/GeunhwaJeong/minting-server) - A scalable system architecture that can process multiple Haneul transactions in parallel using a producer-consumer worker scheme.
- [HaneulInfra](https://haneulnfra.io/) - Provide users and developers with up-to-date recommendations on the ideal RPCs to use for their needs.
- [Haneul RPC Proxy](https://github.com/HaneulSec/haneul-rpc-proxy) - Monitor and analyze the network requests made by the Haneul wallet application and Haneul dApps.
- [PTB Studio](https://ptb.studio) - Visual Programmable Transaction Block Builder.
  - [Documentation](https://haneulcookbook.com/ptb-studio.html)
- [Indexer generator](https://www.npmjs.com/package/haneul-events-indexer) - Code generating tool that will generate an indexer given a smart contract for all the events present. After that the user should remove unwanted events and fix the database schema and handlers (that write to the DB) according to their needs. The tool is written in typescript and uses prisma as an ORM.

### Smart Contract Toolkits

- [Haneul CLI](https://docs.haneul.io/references/cli) - CLI tool to interact with the Haneul network, its features, and the Move programming language.
- [Sentio Debugger](https://docs.sentio.xyz/docs/debugger) - Shows the trace of the transaction [Explorer App](https://app.sentio.xyz/explorer) (mainnet only).
- [`std::debug`](https://docs.haneul.io/guides/developer/first-app/debug#related-links) - Print arbitrary values to the console to help with debugging process.
- [Haneul Tears 💧 (Interest Protocol)](https://docs.interestprotocol.com/overview/haneul-tears) - Open source production ready Haneul Move library to increase the productivity of new and experienced developers alike.
- [Haneul Codec](https://github.com/haneul-potatoes/app/tree/main/packages/codec) - Ultimate encoding solution for Haneul.
- [SkipList (Cetus)](https://github.com/CetusProtocol/move-stl) - A skip link list implement by Move language in Haneul.
- [IntegerMate (Cetus)](https://github.com/CetusProtocol/integer-mate) - A Library of move module provides signed integer and some integer math functions.
- [Cetus CLMM](https://github.com/CetusProtocol/cetus-contracts/tree/main/packages/cetus_clmm) - The Cetus CLMM DEX open-source code. 
- [HaneulDouble Metadata](https://github.com/haneuldouble/haneuldouble_metadata) - A Haneul Move library and a set of tools to store, retrieve, and manage any type of primitive data as chunks in a `vector<u8>`. Store any data in the `vector<u8>` without dependencies and without any `Struct` defined.
- [Move on Haneul examples (Haneul Labs)](https://github.com/GeunhwaJeong/haneul/tree/main/examples/move) - Examples of Move on Haneul applications.
- [HaneulGPT Decompiler](https://haneulgpt.tools/decompile) - Uses generative AI to convert Move bytecode back to source code.
- [Revela](https://revela.verichains.io/) - Decompile Haneul smart contracts to recover Move source code.
- Package Source Code Verification - Verify your package source code on Haneulscan, powered by WELLDONE Studio and Blockberry.
  - [Documentation](https://docs.blockberry.one/docs/contract-verification) - [Form Submission](https://haneulscan.xyz/mainnet/package-verification)
- [Dubhe CLI (Dubhe Engine)](https://github.com/0xobelisk/dubhe/tree/main/packages/haneul-cli) - For building, and managing Dapps built on Dubhe Engine in Haneul.
  - [Documentation](https://dubhe.obelisk.build/dubhe/haneul/cli)
- [Haneul Token CLI RPC](https://github.com/otter-sec/haneul-token-gen-rpc) - A Rust-based RPC service for generating and verifying Haneul token smart contracts effortlessly.
  - [Haneul Token CLI Tool](https://github.com/otter-sec/haneul-token-gen) - A Rust-based Command-Line Interface (CLI) tool designed to simplify the process of generating and verifying Haneul token smart contracts

## Indexers & Data Services

- ZettaBlock - Generate custom GraphQL or REST APIs from SQL queries and incorporate your private off-chain data.
  - [Homepage](https://zettablock.com/) - [Docs](https://docs.zettablock.com) - [Pricing](https://zettablock.com/pricing) - [Further Information](details/indexer_zettablock.md)
- Sentio - Transform raw indexed data (transactions, events, etc.) into meaningful queryable data by writing custom processor logic.
  - [Homepage](https://www.sentio.xyz/indexer/) - [Documentation](https://docs.sentio.xyz/docs/data-collection) - [Examples](https://github.com/sentioxyz/sentio-processors/tree/main/projects) - [Further Information](details/indexer_sentio.md)
- BlockVision - Provide Haneul indexed data for developers through pre-built APIs, such as, Token, NFT, and DeFi, etc.
  - [Homepage](https://blockvision.org/) - [Documentation](https://docs.blockvision.org/reference/welcome-to-blockvision)
- BlockBerry (Haneulscan) - The Blockberry Haneul API provides endpoints that reveal data about significant entities on the Haneul Network. It indexes useful object metadata, including NFTs, domains, collections, coins, etc. Some data is drawn from third-party providers, particularly market data (coin prices, market cap, etc.).
  - [Homepage](https://blockberry.one/) - [Documentation](https://docs.blockberry.one/reference/haneul-quickstart)
- Space And Time (SxT) - Verifiable compute layer for AI x blockchain. Decentralized data warehouse with sub-second ZK proof.
  - [Homepage](https://www.spaceandtime.io/) - [Documentation](https://docs.spaceandtime.io/) - [Further Documentation](details/indexer_space_and_time.md)
- Birdeye Data Services - Access Crypto Market Data APIs on Haneul.
  - [Homepage](https://bds.birdeye.so/) - [Blog](https://blog.haneul.io/birdeye-data-services-crypto-api-websocket/) - [API Documentation](https://docs.birdeye.so/reference/intro/authentication)
- Indexer.xyz (behind TradePort) - The ultimate toolkit for accessing NFT data and integrating trading functionality into your app on Haneul.
  - [Homepage](https://www.indexer.xyz/) - [API Explorer](https://www.indexer.xyz/api-explorer) - [API Docs](https://tradeport.xyz/docs)
- Dubhe Indexer (Dubhe Engine) - Automatic integration with Dubhe Engine, automatic indexing of all events based on Dubhe Engine to build Dapp on Haneul, based on dubhe configuration files.
  - [Homepage](https://github.com/0xobelisk/dubhe/tree/main/packages/haneul-indexer) - [API Documentation](https://dubhe.obelisk.build/dubhe/haneul/indexer)
- <a href="https://surflux.dev"><img alt="Surflux logo" src="media/surflux_logo.svg" width="15" /></a> Surflux - Developer infrastructure for Haneul. Build production-ready apps with powerful APIs, indexing, and real-time data streams.
  - [Homepage](https://surflux.dev/) - [Documentation](https://docs.surflux.dev/) - [Blog](https://surflux.dev/blog)

## Explorers

- HaneulVision - Data analytics covering transactions, wallets, staking, and validators.
  - [Homepage](https://haneulvision.xyz/) - [Documentation](https://docs.blockvision.org/reference/integrate-haneulvision-into-your-dapp) - [Further Information](details/explorer_haneulvision.md)
- Haneulscan - Explorer and analytics platform for Haneul.
  - [Homepage](https://haneulscan.xyz/mainnet/home) - [Documentation](https://docs.blockberry.one/reference/welcome-to-blockberry-api) - [Further Information](details/explorer_haneulscan.md)
- OKLink - Provide fundamental explorer and data APIs on Haneul.
  - [Homepage](https://www.oklink.com/haneul) - [Further Information](details/explorer_oklink.md)
- Polymedia Explorer - A fork of the original Haneul Explorer.
  - [Homepage](https://explorer.polymedia.app) - [GitHub](https://github.com/juzybits/polymedia-explorer) - [Further Information](details/explorer_polymedia.md)
- PTB Explorer - A fork of the Polymedia Explorer.
  - [Homepage](https://explorer.walrus.site/) - [GitHub](https://github.com/zktx-io/polymedia-explorer-ptb-builder)
- Local Haneul Explorer - Haneul Explorer for your localnet maintained by [haneulware](https://github.com/haneulware)
  - [GitHub](https://github.com/haneulware/haneul-explorer) - [Further Information](details/explorer_local_haneul_explorer.md)
- Haneulmon - Powerful command line tool designed to provide detailed dashboards for monitoring the Haneul network.
  - [GitHub](https://github.com/bartosian/haneulmon) - [Further Information](details/explorer_haneulmon.md)

## Oracles

- Pyth Network - Oracle protocol that connects the owners of market data to applications on multiple blockchains including Haneul.
  - [Homepage](https://www.pyth.network/) - [Documentation](https://docs.pyth.network/home) - [Haneul Tutorial](https://docs.pyth.network/price-feeds/use-real-time-data/haneul) - [Further Information](details/oracle_pyth.md)
- Supra Oracles - Oracle protocol to provide reliable data feed.
  - [Homepage](https://supra.com/) - [Haneul Tutorial](https://docs.supra.com/docs/developer-tutorials/move) - [Further Information](details/oracle_supra.md)
- Switchboard - Data feed customization and management.
  - [Documentation](https://docs.switchboard.xyz/docs) - [Further Information](details/oracle_switchboard.md)

## Security

- <a href="https://info.asymptotic.tech/haneul-prover"><img alt="Haneul Prover logo" src="media/prover_logo.svg" width="15" /></a> [Haneul Prover](https://info.asymptotic.tech/haneul-prover) - Prover for doing Formal Verification of Move on Haneul code.
- [HaneulSecBlockList](https://github.com/HaneulSec/HaneulSecBlockList) - Block malicious websites and packages, Identify and hide phishing objects.
- [DryRunTransactionBlockResponsePlus](https://github.com/HaneulSec/DryRunTransactionBlockResponsePlus) - Decorator of `DryRunTransactionBlockResponse`, highlight `SenderChange`.
- [Guardians](https://github.com/haneulet/guardians) - Phishing Website Protection.
- [HoneyPotDetectionOnHaneul](https://github.com/HaneulSec/HoneyPotDetectionOnHaneul) - Detect HoneyPot SCAM on Haneul.

## AI

- ⚠️ [RagPool](https://ragpool.digkas.nl/) - RAG based chat with docs.
- [Cookbook](https://docsbot-demo-git-haneul-cookbookdev.vercel.app/) - Gemini-based RAG built for docs.
- [Atoma](https://atoma.network/) - Developer-focused infrastructure for private, verifiable, and fully customized AI experiences.
- [Eliza](https://github.com/elizaOS/eliza) - Autonomous agents for everyone.

## Infrastructure as Code

- Haneul Terraform Modules - All-in-one solution for deploying, monitoring, and managing HANEUL infrastructure with ease.
  - [GitHub](https://github.com/bartosian/haneul-terraform-modules) - [Further Information](details/iac_haneul_terraform_modules.md)
- [Dubhe Engine (Obelisk Labs)](https://github.com/0xobelisk/dubhe) - Engine for Everyone to Build Intent-Centric Worlds ⚙️ An Open-Source toolchain for Move Applications.
  - [Documentation](https://dubhe.obelisk.build/) - [Further Information](details/engine_dubhe.md)

## Faucets

- [Haneul Faucet](https://faucet.haneul.io/) - Official web faucet for claiming testnet HANEUL, with wallet integration.
- [n1stake](https://faucet.n1stake.com/) - Community web faucet for claiming testnet HANEUL, with wallet integration.
- [Blockbolt](https://faucet.blockbolt.io/) - Community web faucet for claiming testnet HANEUL, with wallet integration.
- HaneulwareFaucetBot - Haneul Faucet Bot for Telegram.
  - [GitHub](https://github.com/haneulware/HaneulwareFaucetBot) - [Telegram Bot](https://t.me/HaneulwareFaucetBot)
- [Haneulware Faucet Chrome Extension](https://github.com/haneulware/haneulware-faucet-extension) - An experimental Chrome extension for receiving devnet and testnet HANEUL.
