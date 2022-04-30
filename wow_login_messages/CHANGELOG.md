# Changelog

Not used for now.

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Additional getters and setters for `CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag`, `CMD_AUTH_LOGON_PROOF_ClientSecurityFlag`, and `RealmRealmFlag`. These types were not really usable before these changes.
* `tokio` and `async-std` support. These are activated through the `async_tokio` and `async_std` feature flags.

### Changed

* `unknown5` for `CMD_AUTH_LOGON_CHALLENGE_Server` is no longer constantly set to 1. For data that is not 100% known it is better for the emulators themselves to experiment and constantly set values.

### Deprecated

### Removed

### Fixed

### Security

## [0.1.0] - 2022-04-03

### Added

* Initial release. This is very WIP and things are not guaranteed to be documented or actually work correctly.

[0.1.0]: https://github.com/gtker/wow_messages/tree/505efadbe332dee2bdd5d321242b7e5d2565d841
