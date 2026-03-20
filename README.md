[Latest Version]: https://img.shields.io/crates/v/mdbook-asciinema.svg
[crates.io]: https://crates.io/crates/mdbook-asciinema

# `mdbook-asciinema` &emsp; [![Latest Version]][crates.io]

An [`asciinema`](https://asciinema.org) preprocessor for [`mdbook`](https://github.com/rust-lang/mdBook).

## Installation

- [Install `mdbook`](https://rust-lang.github.io/mdBook/guide/installation.html)

- Install `mdbook-asciinema`:

  To install the latest release published to [crates.io](https://crates.io/crates/mdbook-asciinema):

  ```sh
  cargo install mdbook-asciinema --locked
  ```

  The install the latest version committed to GitHub:

  ```sh
  cargo install mdbook-asciinema --git https://github.com/s-samadi/mdbook-asciinema.git --locked
  ```

## Getting Started

Create a new book.

```bash
mdbook init --force --title "My Book"
```

Edit the `book.toml` file; add the `asciinema` preprocessor.

```diff
[book]
title = "My Book"

+ [preprocessor.asciinema]
```

Download an asciicast file (i.e. [demo.cast](https://raw.githubusercontent.com/asciinema/asciinema/202d5c5761687b489451e9bb1a5fe9189b73e9d9/tests/casts/demo.cast)) into a path underneath `src`.

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

Add a helper tag to a markdown file (i.e. chapter_1.md).

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
