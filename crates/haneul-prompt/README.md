# `haneul prompt`

`haneul prompt` is the agent-agnostic entry point to expert Move knowledge, shipped as
part of the Haneul CLI: a self-contained, embedded source of skills and workflows
organized into **categories** (`audit`, `bytecode`, …). Each category is a small
workflow that points an AI agent (or a human) at the skills relevant to a kind of
work.

This README is the developer-facing entry point for the subcommand. The text printed at
runtime by `haneul prompt` (no args) lives in `src/prompt-output.md` and is embedded into
the binary at build time via `include_str!`.

## What this subcommand is

- **Contract:** out = markdown (categories, skills, overview). `haneul prompt` itself only
  prints embedded markdown; it never builds, fetches, or writes artifacts. Some workflows
  may instruct follow-up commands when the task requires them.
- **Self-contained.** All category and skill markdown is embedded in the binary at
  build time.
- **Agent-agnostic.** Works for any AI agent that can shell out.

## Install

`haneul prompt` is built into the Haneul CLI. Install `haneul` per the [official Haneul CLI install
guide](https://docs.haneul.io/getting-started/onboarding/haneul-install); once `haneul` is on
your `PATH`, `haneul prompt` is available.

## Commands

### Example 1 — discoverability overview

```sh
haneul prompt
```

Prints `prompt-output.md`, embedded at build time. The overview names the categories and
explains how to navigate to them. An agent reads this first to learn the surface.

### Example 2 — list categories

```sh
haneul prompt categories
```

Prints a list of embedded categories. Each entry includes the category name and
the short frontmatter description used for routing. The list is followed by navigation
commands for reading a category or switching to skill-bundle navigation.

### Example 3 — read a category

```sh
haneul prompt category audit
```

Prints the body of `categories/audit/CATEGORY.md` verbatim — the workflow, the triage discipline, external references, etc. The category body is
where an agent learns *how to do this kind of work*.

### Example 4 — list skill bundles (flat)

```sh
haneul prompt skills
```

Prints a Markdown list of embedded skill bundles. Each entry includes the bundle name and
the number of embedded markdown files. The list is followed by commands for reading the
bundle entry point, listing reference files, or reading a specific reference file.

### Example 5 — read a skill bundle's SKILL.md

```sh
haneul prompt skill haneul-move-security-review
```

SKILL.md is the bundle's entry point. Reading it alone is not enough — drill into the
reference files for the actual content.

### Example 6 — list reference files in a bundle

```sh
haneul prompt skill haneul-move-security-review --list
```

### Example 7 — read a specific reference file

```sh
haneul prompt skill haneul-move-security-review --file access-control
```

### Example 8 — bulk-load a whole bundle in one call

```sh
haneul prompt skill haneul-move-security-review --all
```

Prints `SKILL.md` and every reference file in the bundle, each preceded by a
`# === FILE: <filename> ===` separator. Use when you know you need the whole
bundle and want to avoid one process per reference file. The `=== FILE: ===`
sentinel is greppable and visually distinct from any `#` heading the file's
own content might use.

### Example 9 — bulk-load a whole category in one call

```sh
haneul prompt category audit --all
```

Prints every skill bundle the `audit` category names — `SKILL.md` plus every
reference file each — in one call. Each file is preceded by a
`# === FILE: <bundle>/<filename> ===` separator so source attribution is
preserved and file boundaries don't collide with any `#` heading inside the
file's own content. `CATEGORY.md` itself is NOT re-printed; read it first
with `haneul prompt category audit`. Use `haneul prompt category audit --list` to
see the deep inventory before deciding.

## Worked agent flow

Realistic use case — point any AI agent at the binary with a prompt that names the kind
of work (e.g. *"audit Haneul mainnet package `0x<id>` for security vulnerabilities; execute the
`haneul prompt` binary to find the right skills"*):

1. Agent calls `haneul prompt` — learns the surface.
2. Agent calls `haneul prompt categories` — sees `audit`, `build`, `bytecode`, etc.
3. Agent calls `haneul prompt category audit` — gets the workflow, triage discipline, external references, etc.
4. Agent loads the skill bundles the category names. The fewest round trips is
   `haneul prompt category <name> --all` (one call for the whole category); a single
   bundle is `haneul prompt skill <bundle> --all`; or it can enumerate file by file
   with `--list` + `--file <ref>`.
5. Agent follows the workflow against the target package — fetch via one Haneul GraphQL
   call, disassemble every module with `haneul move disassemble`, walk the SM-* rules
   over the resulting `.asm` files (paired with `auditing-bytecode.md` for the
   per-rule disassembly signal patterns).
6. Agent produces findings in the format the audit category prescribes:
   `SM-ID · module.asm:B<block>@i<index>` with a disassembly excerpt as evidence.

The same shape applies to other categories: read the category's body, walk the skills it
names, do the work.

## How to add a category

A category is a single markdown file plus, optionally, references to existing skills:

1. Create `src/categories/<name>/CATEGORY.md`.
2. Add YAML frontmatter with the required keys:
   ```yaml
   ---
   name: <name>
   description: <one-line description used by `haneul prompt categories`>
   skills:
     - <skill-bundle-1>
     - <skill-bundle-2>
   ---
   ```
   The `skills:` list names skill bundles (directories under `src/skills/`). A
   skill bundle can appear in any number of categories — there's no duplication, the
   bundle's canonical location stays under `src/skills/`.
3. Write the body: a workflow, optionally with a
   "Discipline" or "Reproducibility" section, and an "External references" section if
   useful. Describe what is — not what's planned.
4. Rebuild the Haneul CLI. The build script picks up the new category automatically.
5. Verify: `haneul prompt categories` lists the new entry; `haneul prompt category <name>`
   prints the body.

## How to add a skill bundle

1. Create `src/skills/<bundle>/SKILL.md` (the bundle's entry point) plus any
   per-topic reference files (`<topic>.md`).
2. Optionally reference the new bundle from one or more `CATEGORY.md` frontmatter
   `skills:` lists.
3. Rebuild the Haneul CLI.

## Maintainer-only content

Provenance, refresh tooling, and other content that should not be visible to runtime
agents lives at `src/maintenance/` (see its own `README.md`). `build.rs` walks
only `src/skills/` and `src/categories/`, so anything under
`maintenance/` is excluded from the binary by construction.
