# Conway's Game of Life cli in Rust

## Usage

clone the repo and run `cargo r`

## Sample

![Sample][1]

## Todos

-   [x] initial tui support
-   [x] renaming on gh
-   [x] error handling
-   [x] publishing to crates.io
-   [x] changing to `Canvas` for rendering viewer block
-   [ ] *maybe* the ability to parse `.cells` files

## Acknowledgements

-   The core of this app is adapted from the [Rust-Wasm tutorial](https://rustwasm.github.io/docs/book/).
-   main dependencies:
    -   ratatui: ui
    -   crossterm: ratatui backend

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

[1]: assets/0.3.0_stripes.png "Image of using cgol-cli-rs in Alacritty on Arch Linux"
