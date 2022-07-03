# `wow_vanilla_messages`

Implementation of the network protocol used between World of Warcraft [game servers and clients](https://wowdev.wiki/World_Packet) for game versions 1.0.0 to 1.12.3 (Vanilla).
See the [WoWDev Login page](https://wowdev.wiki/Login) for details.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
wow_vanilla_messages = { version = "0.1" }
```

Or add it with [cargo edit](https://github.com/killercup/cargo-edit):
```bash
cargo add wow_login_messages
```

And then [check out the docs](https://docs.rs/wow_vanilla_messages/latest/).

## Design Decisions

Types have been named the ugly `CMD_SCREAMING_SNAKE_CASE` way because that's
what other (mostly C++) emulators call them. This makes it significantly easier
to search through other emulators or other documentation.

## Other Work

* [vMaNGOS (C++)](https://github.com/vmangos/core/blob/ce164f3eb32c75b244482070fbaf3ada1110e6be/src/realmd/AuthSocket.cpp#L65)
and MaNGOS derivatives in general have a relatively complete list of messages for popular versions,
but they are not available as a library.

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
