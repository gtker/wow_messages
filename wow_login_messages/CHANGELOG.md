# Changelog

Not used for now.

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added

* Flags now have their enumerators as public `const` variables.

### Changed

* BREAKING: Enums will now default to the first enumerator without any fields. If all enumerators have fields the first one will be used.
* BREAKING: Write functions have been changed from taking a `&mut impl Write` to just a `Write` [in accordance with the library guidelines](https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value).
* BREAKING: Read functions have been changed from taking a `&mut impl Read` to just `Read` [in accordance with the library guidelines](https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value).
* BREAKING: `ServerMessage` and `ClientMessage` traits have been sealed to prevent downstream implementation. Read methods have also been sealed.

## [0.2.0] - 2022-10-31

### Added

* Additional getters and setters for `CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag`, `CMD_AUTH_LOGON_PROOF_ClientSecurityFlag`, and `RealmRealmFlag`. These types were not really usable before these changes.
* `tokio` and `async-std` support. These are activated through the `tokio` and `async-std` feature flags.

### Changed

* BREAKING: Enumerators are now PascalCase instead of SCREAMING_SNAKE_CASE.
* BREAKING: Structs with enumerator names in them have also been renamed to use PascalCase. For example `CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_AUTHENTICATOR` is now `CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator`.
* BREAKING: The synchronous Read/Write functions are now behind the `sync` feature flag. This is in order to reduce compile times when only using `tokio`/`async_std`. If using either of those it's highly unlikely that the synchronous versions will be needed.
* `unknown5` for `CMD_AUTH_LOGON_CHALLENGE_Server` is no longer constantly set to 1. For data that is not 100% known it is better for the emulators themselves to experiment and constantly set values.
* BREAKING: `read_expect_*_login_message`s have been renamed to `expect_*_message` for brevity.
`Login` has been removed from the names of types in `helper` as well.
* BREAKING: Added a single `EnumError` type instead of a separate type for every single enum.
* BREAKING: `Expected*MesageError`s have been renamed to `ExpectedMessageError` along with changes to internal structure.
* BREAKING: Flags setters now always take `mut self` instead of `&mut self`. This makes the builder pattern the only correct way of instantiating flag objects.
* BREAKING: Composite type names are now separated by an underscore instead of nothing. This should improve readability.
* BREAKING: Renaming `login_result` variables to just `result` in order to be consistent with other messages.

### Removed

* BREAKING: `SecurifyFlag` enum. This was never used except in the `CMD_AUTH_LOGON_PROOF_ClientSecurityFlag` version.
* BREAKING: Several member functions on enums, flags, and structs that were not useful for library users.
* BREAKING: `MessageBody` trait. Functionality has been moved into `ClientMessage` and `ServerMessage` for simplicity.
* BREAKING: `ClientOpcode` and `ServerOpcode` for all versions.
This functionality exposes a difference between the opcode and the body in a way that is inconsistent with the abstractions of the crate.

## [0.1.0] - 2022-04-03

### Added

* Initial release. This is very WIP and things are not guaranteed to be documented or actually work correctly.

<!-- next-url -->
[Unreleased]: https://github.com/gtker/wow_messages/compare/wow_login_messages-v0.2.0...HEAD
[0.2.0]: https://github.com/gtker/wow_messages/releases/tag/wow_login_messages-v0.2.0
[0.1.0]: https://github.com/gtker/wow_messages/tree/505efadbe332dee2bdd5d321242b7e5d2565d841
