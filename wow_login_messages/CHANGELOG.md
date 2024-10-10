# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate


## [0.5.0] - 2024-10-10

### Changed

* BREAKING: `ServerMessage` and `ClientMessage` turned into marker traits with shared functionality moved into `Message`
  trait.
* BREAKING: Renamed `unknown_flags` on version 8 `CMD_AUTH_LOGON_PROOF_Server` to `unknown` for consistency with other
  messages.
* BREAKING: Protocol version 5, 6, and 7 now also have matrix cards.
* BREAKING: `CMD_AUTH_LOGON_PROOF_Client` field `authenticator` is now a `String` from `Vec<u8>`.
* BREAKING: Version 5 `CMD_REALM_LIST_Server` is now the same as version 6/7.
* BREAKING: Message structs that only consisted of a single field with an enum have been converted to enums.
* BREAKING: `utc_timezone_offset` in `CMD_AUTH_LOGON_CHALLENGE_Client`/`CMD_AUTH_RECONNECT_CHALLENGE_Client` is now a signed integer.

## [0.4.0] - 2024-02-21

### Added

* `std::fmt::Display` for new enums.
* `supports_pin`/`supports_matrix_card`/`supports_authenticator` functions for `ProtocolVersion` and `Version`.

### Changed

* BREAKING: `protocol_version` for `CMD_AUTH_RECONNECT_CHALLENGE_Client` and `CMD_AUTH_LOGON_CHALLENGE_Client` has been
  converted to enum named `ProtocolVersion`.
* BREAKING: `Locale`, `Os`, and `Platform` no longer have an `Other` enumerator.
* BREAKING: Opcode types no longer implement `Eq`.
* BREAKING: `Population::Other` changed from `u32` to `f32`.

### Removed

* BREAKING: Removed `print-testcase` feature and all associated functionality. All known versions have been identified
  and tests have been added.

## [0.3.0] - 2023-05-20

### Added

* Flags now have their enumerators as public `const` variables.
* Login and reconnect protocol version 5, 6, and 7.
* BREAKING: TryFrom for all enums and flags for `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, and `usize`
  regardless of underlying type.
  This may break type deduction and lead to compile errors, so it is breaking.

### Changed

* BREAKING: `ParseError` enum has been renamed to `ParseErrorKind`. `ParseError` is now a struct that holds more
  information about the error, such as opcode and size.
* BREAKING: Enums will now default to the first enumerator without any fields. If all enumerators have fields the first
  one will be used.
* BREAKING: Write functions have been changed from taking a `&mut impl Write` to just
  a `Write` [in accordance with the library guidelines](https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value).
* BREAKING: Read functions have been changed from taking a `&mut impl Read` to
  just `Read` [in accordance with the library guidelines](https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value).
* BREAKING: `ServerMessage` and `ClientMessage` traits have been sealed to prevent downstream implementation. Read
  methods have also been sealed.
* BREAKING: The `locked` field of `Realm` for login version 8 is now a `bool` instead of a `u8`.
* BREAKING: All flags have had their previously `ALL_CAPS` names changed to lower case.
* BREAKING: `EnumError` `value` field has been changed from `u64` to `i128`. This is in order to be able to hold all
  possible TryFrom values.

## [0.2.0] - 2022-10-31

### Added

* Additional getters and setters
  for `CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag`, `CMD_AUTH_LOGON_PROOF_ClientSecurityFlag`, and `RealmRealmFlag`.
  These types were not really usable before these changes.
* `tokio` and `async-std` support. These are activated through the `tokio` and `async-std` feature flags.

### Changed

* BREAKING: Enumerators are now PascalCase instead of SCREAMING_SNAKE_CASE.
* BREAKING: Structs with enumerator names in them have also been renamed to use PascalCase. For
  example `CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_AUTHENTICATOR` is
  now `CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator`.
* BREAKING: The synchronous Read/Write functions are now behind the `sync` feature flag. This is in order to reduce
  compile times when only using `tokio`/`async_std`. If using either of those it's highly unlikely that the synchronous
  versions will be needed.
* `unknown5` for `CMD_AUTH_LOGON_CHALLENGE_Server` is no longer constantly set to 1. For data that is not 100% known it
  is better for the emulators themselves to experiment and constantly set values.
* BREAKING: `read_expect_*_login_message`s have been renamed to `expect_*_message` for brevity.
  `Login` has been removed from the names of types in `helper` as well.
* BREAKING: Added a single `EnumError` type instead of a separate type for every single enum.
* BREAKING: `Expected*MesageError`s have been renamed to `ExpectedMessageError` along with changes to internal
  structure.
* BREAKING: Flags setters now always take `mut self` instead of `&mut self`. This makes the builder pattern the only
  correct way of instantiating flag objects.
* BREAKING: Composite type names are now separated by an underscore instead of nothing. This should improve readability.
* BREAKING: Renaming `login_result` variables to just `result` in order to be consistent with other messages.

### Removed

* BREAKING: `SecurifyFlag` enum. This was never used except in the `CMD_AUTH_LOGON_PROOF_ClientSecurityFlag` version.
* BREAKING: Several member functions on enums, flags, and structs that were not useful for library users.
* BREAKING: `MessageBody` trait. Functionality has been moved into `ClientMessage` and `ServerMessage` for simplicity.
* BREAKING: `ClientOpcode` and `ServerOpcode` for all versions.
  This functionality exposes a difference between the opcode and the body in a way that is inconsistent with the
  abstractions of the crate.

## [0.1.0] - 2022-04-03

### Added

* Initial release. This is very WIP and things are not guaranteed to be documented or actually work correctly.

<!-- next-url -->
[Unreleased]: https://github.com/gtker/wow_messages/compare/wow_login_messages-v0.5.0...HEAD

[0.5.0]: https://github.com/gtker/wow_messages/compare/wow_login_messages-v0.4.0...wow_login_messages-v0.5.0

[0.4.0]: https://github.com/gtker/wow_messages/compare/wow_login_messages-v0.3.0...wow_login_messages-v0.4.0

[0.3.0]: https://github.com/gtker/wow_messages/compare/wow_login_messages-v0.2.0...wow_login_messages-v0.3.0

[0.2.0]: https://github.com/gtker/wow_messages/releases/tag/wow_login_messages-v0.2.0

[0.1.0]: https://github.com/gtker/wow_messages/releases/tag/wow_login_messages-v0.1.0
