# `wow_messages`

World of Warcraft authentication + game server message definitions for Rust.
Auto generated from the `wowm` language found in `wow_message_parser/wowm`.

The project is split into the subdirectories:

* `wow_login_messages`: Rust library for the authentication
  server. [On crates.io](https://crates.io/crates/wow_login_messages).
* `wow_message_parser`: The parser, codegen and definitions that creates the libraries.
  Used to build `wow_login_messages`, `wow_world_messages` and `wowm_language`.
* `wow_world_messages`: Rust library for the world server for WoW version 1.x.y.
* `wow_world_base`: Rust library that provides the basic types and enums such as `Class`, `Race`
  and `Gender` for `wow_world_messages` and others. [On crates.io](https://crates.io/crates/wow_world_base).
* `wow_items`: Rust library that contains an array of all `Item`s. Uses the `Item` type
  from `wow_world_base`. [On crates.io](https://crates.io/crates/wow_items).
* `wow_spells`: Rust library that contains an array of all `Spell`s.
  Uses the `Spell` type from `wow_world_base`.
  [On crates.io](https://crates.io/crates/wow_spells).
* `wowm_language`: [mdbook](https://github.com/rust-lang/mdBook) of language specification. Hosted
  at [`www.gtker.com/wow_messages`](https://www.gtker.com/wow_messages).
* `intermediate_representation.json`: contains a machine readable version of the `wowm` files.
* `intermediate_representation_schema.json`: a [JSON type def](https://jsontypedef.com/) schema for
  the `intermediate_representation.json`.
* `examples` Example servers and clients that will work with 1.12, 2.4.3 and 3.3.5 clients.

Run `cargo gen && cargo test` to "compile" all libraries and ensure that there are no issues.

Run `cargo overwrite` to overwrite the current regression test files with new output.

The following environment variables can be used with the parser:

* `WOWM_WIRESHARK`: must point to a valid `packet-woww.c` in a wireshark repo. This file will then be modified with the
  auto generated Wireshark messages.
* `WOWM_SQLITE_DB_PATH`: `wow_db_sqlite` directory. Writes the types and functions for `wow_world_base` `extended`.
* `WOWM_PRINT_TEST_ERRORS`: when set errors testing that the application exists with a message will print error
  messages.
* `WOWM_ONLY_PRINT_NAME_OF_SINGLE_MESSAGE`: when set the reporting output will be the first unimplemented message
  without a newline character. This is intended for quickly implementing many messages in quick succession.

## Additional libraries

* [`wow_messages_python`](https://github.com/gtker/wow_messages_python), message definitions in a Python library.

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
