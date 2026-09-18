# Contributing to This Project

Thanks for considering making a contribution to the Haneul network or its documentation. Before you get started, please take a moment to read these guidelines.

## Important Note

We appreciate contributions, but **simple typo fixes (e.g., minor spelling errors, punctuation changes, or trivial rewording) will be ignored** unless they significantly improve clarity or fix a critical issue. If you are unsure whether your change is substantial enough, consider opening an issue first to discuss it.

# How to Contribute


## Documentation

Found a small error or typo in the main documentation? Each page on the [docs.haneulfoundation.org](https://docs.haneulfoundation.org/) site includes an **Edit this page** link at the bottom that you can use to edit the page in GitHub. The content is located in the docs/content directory of the Haneul repo, so you can make a PR if you prefer. 

For larger documentation issues, you can [create an issue](https://github.com/GeunhwaJeong/haneul/issues/new/choose) in GitHub. To fix the problem yourself, follow the [documentation contribution](./docs/content/references/contribute/contribution-process.mdx) guidelines.

If you change functionality, update the relevant documentation.

Include examples where applicable.

## Reporting Issues

Found a bug or security vulnerability? Please check the existing issues before opening a new one.

Provide as much detail as possible, including steps to reproduce the issue, expected behavior, and actual behavior.

## New Haneul features

If you want to contribute code that creates a feature on Haneul, start by [opening an issue](https://github.com/GeunhwaJeong/haneul/issues/new/choose) that describes the proposal, so the design can be discussed before developing the logic.

## New Haneul Framework Contributions

Want to contribute to the Haneul framework? See [CONTRIBUTING.md](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/CONTRIBUTING.md) for information related to `haneul-framework` crate contributions.

## Proposing Code Changes

Fork the repository and create a new branch for your changes. Ensure your branch is based on the latest main

Follow the coding style and conventions used in the project.

If your change is significant, please open an issue first to discuss it.

## Submitting a Pull Request

Ensure your changes are well-tested. Provide a clear description of your changes in the pull request.

Reference any relevant issue numbers in your pull request. Be responsive to feedback from maintainers.

Name the pull request after the paths it changes, followed by a short lowercase summary:

```
<path>[, <path>...]: <summary>
```

For example, `crates/haneul-core: retry consensus submissions on transient errors` or `consensus, crates/haneul-node: share the block verifier between services`. List only the directories with functional changes; a change that trickles through the whole tree names the parent directory instead, such as `crates: upgrade to protocol version 127`. Changes with no natural path use `chore`, `build`, `ci` or `test` as the prefix. Pull requests are squash-merged, so the title becomes the commit subject on `main`.

## Code Standards

Follow existing code structure and formatting.

Write meaningful commit messages. Commit subjects use the same path-prefixed format as pull request titles, and the body explains why the change is correct.

Ensure all tests pass before submitting a pull request.

# License

By contributing, you agree that your contributions will be licensed under the same license as this project.

Thank you for contributing!
