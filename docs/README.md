# The Haneul documentation has moved

The pages behind [docs.haneul.io](https://docs.haneul.io) are no longer in this
repository. They now live in
**[HaneulLabs/haneul-docs](https://github.com/GeunhwaJeong/haneul-docs)**, along with the
Docusaurus site that renders them. The site itself is unchanged: same URLs, same
content.

| What you want | Where it is now |
| --- | --- |
| A documentation page | [`content/haneul/`](https://github.com/GeunhwaJeong/haneul-docs/tree/main/content/haneul) |
| The site, its theme and plugins | [`sites/haneul/`](https://github.com/GeunhwaJeong/haneul-docs/tree/main/sites/haneul) |
| To report a problem with the docs | [Open an issue](https://github.com/GeunhwaJeong/haneul-docs/issues/new) on `haneul-docs` |
| To fix a page yourself | The **Edit this page** link at the bottom of any page on docs.haneul.io, or a pull request against `haneul-docs` |

## What is still here, and why

Parts of the documentation are generated at build time from source in this
repository, and `haneul-docs` fetches them on every build:

| Path | Generates |
| --- | --- |
| `crates/haneul-framework/docs/` | the Move framework reference |
| `crates/`, `examples/` | the code embedded in pages by `ImportContent` |
| `release-notes/` | the release notes page |

Two things follow.

Comments marked `docs::#name` in Rust and Move source are **snippet anchors**, not
ordinary comments. A page quotes the lines between them, so renaming or removing
one breaks whatever page embeds it.

Moving or renaming any of the paths above breaks the documentation build in the
other repository, and nothing here will tell you. If you need to move one,
change
[`sources.json`](https://github.com/GeunhwaJeong/haneul-docs/blob/main/sources.json)
in `haneul-docs` in the same change.

The awesome-haneul lists used to be vendored here as a git subtree under
`docs/subtree`. `haneul-docs` now fetches them from
[haneul-foundation/awesome-haneul](https://github.com/haneul-foundation/awesome-haneul) and
[becky-haneul/awesome-haneul-gaming](https://github.com/becky-haneul/awesome-haneul-gaming)
directly, so there is nothing to keep in sync here.

## Why it moved

The machinery underneath seven documentation sites had been copied rather than
shared: the same fetcher written three times, the same frontmatter schema in
three repositories, shared components kept aligned across four repositories by a
sync bot. `haneul-docs` gives that one home.
