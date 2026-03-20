# Example

A minimal [mdbook](https://rust-lang.github.io/mdBook/) project that uses the `mdbook-asciinema` preprocessor to embed an [asciinema](https://asciinema.org) terminal recording.

## Prerequisites

- [mdbook](https://rust-lang.github.io/mdBook/guide/installation.html)
- `mdbook-asciinema` (install from the repo root: `cargo install --path mdbook-asciinema --locked`)

## Usage

```sh
cd example
mdbook serve
```

Then open <http://localhost:3000> to see the embedded player in action.
