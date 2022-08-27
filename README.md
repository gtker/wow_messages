# `wow_messages`

World of Warcraft authentication + game server message definitions for Rust.
Auto generated from the `wowm` language found in `wow_message_parser/wowm`.

The project is split into the subdirectories:

* `wow_login_messages`: Rust library for the authentication
  server. [On crates.io](https://crates.io/crates/wow_login_messages).
* `wow_message_parser`: The parser, codegen and definitions that creates the libraries.
  Used to build `wow_login_messages`, `wow_world_messages` and `wowm_language`.
* `wow_world_messages`: **UNRELEASED** Rust library for the world server for WoW version 1.x.y.
* `wow_world_base`: **UNRELEASED** Rust library that provides the basic types and enums such as `Class`, `Race`
  and `Gender` for `wow_world_messages` and others.
* `simple_server`: Test server that uses the authentication library. Can work as a start to making your own server. Uses
  tokio.
* `simple_client`: Test client that uses the authentication library. Can work as a start to making your own client. Uses
  sync (std:io::Read/Write).
* `wowm_language`: [mdbook](https://github.com/rust-lang/mdBook) of language specification. Hosted
  at [`www.gtker.com/wow_messages`](https://www.gtker.com/wow_messages).
* `intermediate_representation.json`: **UNRELEASED** contains a machine readable version of the `wowm` files.

Run `cargo run -p wow_message_parser && cargo test` to "compile" all libraries and ensure that there are no issues.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
