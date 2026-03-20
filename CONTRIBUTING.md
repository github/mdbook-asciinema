## Contributing

[fork]: https://github.com/github/mdbook-asciinema/fork
[pr]: https://github.com/github/mdbook-asciinema/compare

Hi there! We're thrilled that you'd like to contribute to this project. Your help is essential for keeping it great.

Contributions to this project are [released](https://help.github.com/articles/github-terms-of-service/#6-contributions-under-repository-license) to the public under the [project's open source license](LICENSE).

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## Prerequisites for running and testing code

These are one time installations required to be able to test your changes locally as part of the pull request (PR) submission process.

1. Install Rust [through rustup](https://rustup.rs/) (stable toolchain)
1. Install [mdbook](https://rust-lang.github.io/mdBook/guide/installation.html): `cargo install mdbook --locked`

## Submitting a pull request

1. [Fork][fork] and clone the repository
1. Build the project: `cargo build`
1. Make sure the tests pass on your machine: `cargo test`
1. Make sure linting passes on your machine: `cargo clippy -- -D warnings`
1. Make sure formatting is correct: `cargo fmt -- --check`
1. Create a new branch: `git checkout -b my-branch-name`
1. Make your change, add tests, and make sure the tests, linter, and formatting still pass
1. Push to your fork and [submit a pull request][pr]
1. Pat yourself on the back and wait for your pull request to be reviewed and merged.

Here are a few things you can do that will increase the likelihood of your pull request being accepted:

- Follow standard [Rust API guidelines](https://rust-lang.github.io/api-guidelines/) and idiomatic Rust style.
- Write tests.
- Keep your change as focused as possible. If there are multiple changes you would like to make that are not dependent upon each other, consider submitting them as separate pull requests.
- Write a [good commit message](http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html).

## Resources

- [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
- [Using Pull Requests](https://help.github.com/articles/about-pull-requests/)
- [GitHub Help](https://help.github.com)