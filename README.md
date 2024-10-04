# Conway's Game of Life TUI in Rust

## Usage

installation methods

-   `cargo install cgol-tui`
-   `cargo install --locked --git "https://github.com/JeromeSchmied/cgol-tui-rs"`
-   clone the repo and run `cargo install --locked --path .`

after

`[curl "https://conwaylife.com/patterns/<pattern>.cells" | ] cgol-tui [[-],<pattern>.cells,...]`
eg.:

-   `cgol-tui`
-   `curl https://conwaylife.com/patterns/fx153.cells | cgol-tui -` the `-` stands for `stdin`
-   `cgol-tui my_own_pattern.cells fx153.cells`

### Script

there is a fish script provided under [scripts](./scripts/pattern.fish) for viewing patterns from [conwaylife.com](https://conwaylife.com/patterns)</br>
usage: `pattern.fish <PATTERN_NAME> [OPTIONS]`</br>
PATTERN_NAME: either a name of a pattern, or nothing to list all patterns</br>
OPTIONS: -d, --download     download to /tmp

#### needed tools

- [fish](https://fishshell.com/): shell
- [curl](https://curl.se/): download
- [ripgrep](https://github.com/BurntSushi/ripgrep): only for listing all patterns

## Sample

![Sample][1]

## Todos

-   [x] initial tui support
-   [x] renaming on gh
-   [x] error handling
-   [x] publishing to crates.io
-   [x] changing to `Canvas` for rendering viewer block
-   [x] the ability to parse `.cells` files, from [conwaylife.com](https://conwaylife.com/patterns)
-   [x] display the names of patterns

## Acknowledgements

-   The core of this app is adapted from the [Rust-Wasm tutorial](https://rustwasm.github.io/docs/book/).
-   main dependencies:
    -   [ratatui](https://ratatui.rs): ui
    -   [crossterm](https://github.com/crossterm-rs/crossterm): ratatui backend

## License

Licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[1]: assets/0.4.0.png "Image of using cgol-tui in Alacritty on Arch Linux btw"
