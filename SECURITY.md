# Security Policy

## Reporting a Vulnerability

We appreciate your help in keeping the Haneul network secure. If you believe you have found a security vulnerability, please report it privately through [GitHub's private vulnerability reporting](https://github.com/GeunhwaJeong/haneul/security/advisories/new). **Do not report security issues through public GitHub issues, discussions, or pull requests.**

We will make every effort to acknowledge your report promptly, and we will keep you informed while a fix is being worked on. Please do not disclose the issue publicly until a fix has been released.

Please do not test against the Haneul mainnet. Issues can be reproduced on a local network instead (`haneul start --force-regenesis`).

## Areas of particular concern

Reports in the following areas are especially valuable:

- Loss or theft of funds, including unauthorized creation, copying, transfer or destruction of objects
- Exceeding the maximum supply of 10 billion HANEUL
- Violating BFT assumptions, or otherwise compromising the integrity of proof of stake governance
- Unintended chain splits or network halts that would require a hard fork to resolve
- Remote code execution on unmodified validator software
- Remote calls that crash a validator or fullnode

## Out of scope

- Attacks requiring leaked keys or privileged access
- Denial of service and traffic-volume attacks
- Centralization concerns or best-practice critiques without a concrete vulnerability

## Bug bounty

A bug bounty program is planned but not yet operating. Until it launches there are no monetary rewards, but reporters of valid vulnerabilities will be credited in the release notes of the fix, unless they prefer to remain anonymous.
