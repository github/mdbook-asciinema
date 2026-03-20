[Latest Version]: https://img.shields.io/crates/v/mdbook-asciinema.svg
[crates.io]: https://crates.io/crates/mdbook-asciinema

# `mdbook-asciinema` &emsp; [![Latest Version]][crates.io] [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/github/mdbook-asciinema/blob/main/LICENSE)

An [`mdbook`](https://github.com/rust-lang/mdBook) preprocessor that embeds [`asciinema`](https://asciinema.org) terminal recordings directly into your book pages using the [asciinema-player](https://docs.asciinema.org/manual/player/).

## Installation

- [Install `mdbook`](https://rust-lang.github.io/mdBook/guide/installation.html)

- Install `mdbook-asciinema`:

  To install the latest release published to [crates.io](https://crates.io/crates/mdbook-asciinema):

  ```sh
  cargo install mdbook-asciinema --locked
  ```

  To install the latest version committed to GitHub:

  ```sh
  cargo install mdbook-asciinema --git https://github.com/s-samadi/mdbook-asciinema.git --locked
  ```

## Getting Started

Create a new book.

```bash
mdbook init --force --title <TITLE>
```

Edit the `book.toml` file; add the `asciinema` preprocessor.

```diff
[book]
title = "My Book"

+ [preprocessor.asciinema]
```

Add your asciicast file/s into a path underneath `src`.

```
PROJ_DIR
├── book
├── book.toml
└── src
    ├── chapter_1.md
    ├── SUMMARY.md
    └── video
        └── demo.cast
```

Add a helper tag to a markdown file (e.g. `chapter_1.md`).

```md
# Chapter 1

{{ #asciinema video/demo.cast }}

```

See the [example](example/) directory for a ready-to-run setup.

Preview the book by running [mdbook serve](https://rust-lang.github.io/mdBook/cli/serve.html).

```bash
> mdbook serve
 INFO Book building has started
 INFO mdbook_asciinema: created "[PROJ_DIR]/src/lib/asciinema-player/asciinema-player.min.js"
 INFO mdbook_asciinema: created "[PROJ_DIR]/src/lib/asciinema-player/asciinema-player.css"
 INFO Running the html backend
 INFO HTML book written to `[PROJ_DIR]/book`
 INFO Serving on: http://localhost:3000
 INFO Watching for changes...
```

## Syntax

The general form of the helper tag is:

```md
{{ #asciinema <path> [opts=<path>] [encoding=<value>] [parser=<value>] }}
```

| Parameter  | Required | Description |
|------------|----------|-------------|
| `path`     | Yes      | Relative path (from `src`) to a `.cast` asciicast file. |
| `opts`     | No       | Relative path to a JSON file containing [player options](https://docs.asciinema.org/manual/player/options/). |
| `encoding` | No       | Character encoding of the cast (e.g. `utf-8`). |
| `parser`   | No       | Parser to use for the cast (e.g. `asciicast`). |

### Player Options

To customise playback, create a JSON file with [asciinema-player options](https://docs.asciinema.org/manual/player/options/) and reference it with the `opts` parameter:

```json
{
  "autoPlay": true,
  "speed": 2,
  "idleTimeLimit": 0.5,
  "cols": 400,
  "rows": 51,
  "fit": false,
  "terminalFontSize": "15px"
}
```

```md
{{ #asciinema video/demo.cast opts=video/player-opts.json }}
```

### Escaping

To display the helper tag literally without rendering it, prefix it with a backslash:

```md
\{{ #asciinema video/demo.cast }}
```

## How It Works

When `mdbook` builds or serves the book, this preprocessor:

1. Copies the bundled asciinema-player JavaScript and CSS assets into `src/lib/asciinema-player/`.
2. Scans each chapter for `{{ #asciinema ... }}` tags.
3. Replaces each tag with the HTML and JavaScript needed to render an embedded asciinema-player.

## Verifying Release Artifacts

All release binaries include [build provenance attestations](https://docs.github.com/en/actions/security-for-github-actions/using-artifact-attestations/using-artifact-attestations-to-establish-provenance-for-builds) and an [SBOM (Software Bill of Materials)](https://docs.github.com/en/actions/security-for-github-actions/using-artifact-attestations/using-artifact-attestations-to-establish-provenance-for-builds#about-sbom-attestations) in SPDX format.

To verify the provenance of a downloaded binary:

```sh
gh attestation verify <artifact-file> --repo github/mdbook-asciinema
```

To verify the SBOM attestation:

```sh
gh attestation verify <artifact-file> --repo github/mdbook-asciinema --predicate-type https://spdx.dev/Document
```

## License

Licensed under the [MIT license](LICENSE).
