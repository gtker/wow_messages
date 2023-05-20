# `wow_items`

Crate containing item definitions for World of Warcraft version 1.12.x, 2.4.3.8606, and 3.3.5.x.

## Usage

Add the crate with the required features:

 ```bash
 cargo add --features 'vanilla tbc wrath' wow_items
 ```

Then [read the docs](https://crates.io/crates/wow_items).

## Notice

This crate contains very large constant arrays which can cause out-of-memory errors during compilation.
Try reducing the amount of cores used for compilation if this is the case.

## Auto Generation

This crate is partially auto generated through sqlite databases in the
[`wow_messages` repository](https://github.com/gtker/wow_messages/).

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
