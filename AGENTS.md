# Repository Guidelines

Check the root `CLAUDE.md` file for repo defaults.
Consult crate-specific `CLAUDE.md` or `AGENTS.md` files when changing files in those crates.

## Commit Message Format

Commit subjects are prefixed with the path(s) they modify, followed by a short lowercase description:

```
<path(s)>: description
```

Examples:
- `crates/haneul-core: retry consensus submissions on transient errors`
- `consensus, crates/haneul-node: share the block verifier between services`
- `docs: hide products that have no counterpart on Haneul`

Use comma-separated paths when multiple areas are affected. Only name the directories with functional changes; a change that trickles through the whole tree names the parent directory instead, such as `crates: upgrade to protocol version 127`. Changes with no natural path use `chore`, `build`, `ci` or `test` as the prefix. The body explains what the problem was, why the change is correct and how it was verified.

## Pull Request Title Format

Pull request titles follow the same convention as commit subjects. Pull requests are squash-merged, so the title becomes the commit subject on `main` and GitHub appends the pull request number. Do not put the number in the title yourself.
