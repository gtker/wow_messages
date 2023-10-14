# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

* `AuctionHouse` type for Vanilla, TBC and Wrath with member functions `deposit_percentage`, `cut_percentage`,
  and `(vanilla|tbc|wrath)_faction`.
* `Octal`, `Binary`, `UpperHex`, `LowerHex` for `Guid`.
* `to_emote` for `TextEmote`.
* BREAKING: TryFrom for all enums and flags for `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, and `usize` regardless of underlying type.
 This may break type deduction and lead to compile errors, so it is breaking.
* `ItemSlot` for TBC/Wrath.

### Removed

* BREAKING: `read` and `write_into_vec` methods for `shared` types.
* `write_packed_guid_into_vec`, `size`, `read_packed` methods for `Guid`.
  These were only intended to be used by `wow_world_messages` for reading `Guid`s on the wire and should provide no
  functionality for users for either `wow_world_messages` or `wow_world_base`.

### Changed

* BREAKING: `EnumError` `value` changed from `u64` to `i128`. This is so that all valid `TryFrom` values can fit.
* BREAKING: `CMSG_SWAP_INV_ITEM` fields changed from `u8` to `ItemSlot` for TBC/Wrath.
* BREAKING: Fix some shared types having modules with `vanilla_vanilla` instead of just `vanilla`.
* BREAKING: Fix some types that were in `shared` when they shouldn't have been.
* BREAKING: `Expansion` member `WrathOfTheLichLing` renamed to `WrathOfTheLichKing`.

## [0.1.1] - 2023-05-20

### Added

* `README.md` file.

## [0.1.0] - 2023-05-20

### Added

* First release.

<!-- next-url -->

[Unreleased]: https://github.com/gtker/wow_messages/compare/wow_world_base-v0.1.1...HEAD

[0.1.1]: https://github.com/gtker/wow_messages/compare/wow_world_base-v0.1.0...wow_world_base-v0.1.1

[0.1.0]: https://github.com/gtker/wow_messages/releases/tag/wow_world_base-v0.1.0
