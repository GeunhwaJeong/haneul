The haneul-test-validator starts a local network that includes a Haneul Full node, a Haneul validator, a Haneul faucet and (optionally)
an indexer.

## Guide

Refer to [local network](https://docs.haneul.io/getting-started/onboarding/local-network)

## Run with a persisted state
You can combine this with indexer runs as well to save a persisted state on local development.

1. Generate a config to store db and genesis configs `haneul genesis -f --with-faucet --working-dir=[some-directory]`
2. `haneul-test-validator --config-dir [some-directory]`
