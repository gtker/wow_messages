# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

* `Copy` for `InspectTalentGearMask`.
* `const` for `InspectTalentGearMask::inspect_talent_gears` and `InspectTalentGearMask::size`.

### Changed

* BREAKING: `Server/ClientOpcodeMessage` now boxes any types larger than 8 bytes in order to avoid breaking the stack.
* BREAKING: Changed `BattlegroundBracket::Fourties` to `BattlegroundBracket::Forties`.
* BREAKING: Changed `BuyResult::NotEnoughtMoney` to `BuyResult::NotEnoughMoney`.
* BREAKING: Changed `Skill::Swiming` to `Skill::Swimming`.

### Removed

* BREAKING: `Aura` types for TBC/Wrath.

## [0.2.0] - 2024-02-21

### Added

* The remaining `expected` functions.
* BREAKING: `set_player_field_inv` method on `UpdatePlayer` and `UpdatePlayerBuilder` for vanilla that takes
  an `ItemSlot`. This replaces the previously named functions for specific positions.
* BREAKING: Additional fields for vanilla `ItemSlot`.
* `From` conversion for update mask builder types to proper types.
* `From` conversion for update mask types to `UpdateMask` type.
* `Guid::from_u32s` and `Guid::to_u32s` to create a Guid from high/low `u32`s.
* `Octal`, `Binary`, `UpperHex`, `LowerHex` for `Guid`.
* `std::fmt::Display` for new enums.
* BREAKING: TryFrom for all enums and flags for `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, and `usize`
  regardless of underlying type.
  This may break type deduction and lead to compile errors, so it is breaking.
* `DateTimeError` type for `DateTime` parsing errors.
* `Display` for `DateTime`, `Month`, and `Weekday`.

### Changed

* BREAKING: Fixed incorrect minimum size for `PackedGuid` leading to correct messages reporting errors.
* BREAKING: Fix some shared types having modules with `vanilla_vanilla` instead of just `vanilla`.
* BREAKING: Fix some types that were in `shared` when they shouldn't have been.
* BREAKING: Compressed arrays no longer have a manual decompressed size field.
* BREAKING: `ParseError` enum has been renamed to `ParseErrorKind`. `ParseError` is now a struct that holds more
  information about the error, such as opcode and size.
* BREAKING: `MSG_AUCTION_HELLO_Server`, `SMSG_AUCTION_BIDDER_NOTIFICATION`, and `ReceivedMail` have had the
  variable `auction_house_id` of type `u32` changed to `auction_house` of type `AuctionHouse` enum.
* BREAKING: Vanilla `UpdatePlayer` methods `VISIBLE_ITEM_*` have been replaced by a single `VISIBLE_ITEM` method.
* BREAKING: `CMSG_TEXT_EMOTE` and `SMSG_TEXT_EMOTE` field `emote` changed from `Emote` to `u32`.
* BREAKING: `EnumError` `value` changed from `u64` to `i128`. This is so that all valid `TryFrom` values can fit.
* BREAKING: `DateTime::try_from` now returns `DateTimeError` instead of `EnumError`.
* BREAKING: `DateTime::try_from` will now reject more invalid dates and times.
* BREAKING: Added `DateTime` to `ParseErrorKind`.
* BREAKING: `CacheMask` type is no longer an enum but a struct that also contains the `data` member
  from `SMSG_ACCOUNT_DATA_TIMES`.
* BREAKING: `Expansion` member `WrathOfTheLichLing` renamed to `WrathOfTheLichKing`.

### Removed

* `write_packed_guid_into_vec`, `size`, `read_packed` methods for `Guid`.
  These were only intended to be used by `wow_world_messages` for reading `Guid`s on the wire and should provide no
  functionality for users for either `wow_world_messages` or `wow_world_base`.
* `*object_type` functions for TBC and Wrath `UpdateMask`s.
* BREAKING: `Copy` from `InspectTalentGearMask`. This type took up significant stack space.
* BREAKING: `const` from `InspectTalentGearMask::size` and `*::inspect_talent_gears`.

## [0.1.0] - 2023-05-20

### Added

* First release.

<!-- next-url -->

[Unreleased]: https://github.com/gtker/wow_messages/compare/wow_world_messages-v0.2.0...HEAD

[0.2.0]: https://github.com/gtker/wow_messages/compare/wow_world_messages-v0.1.1...wow_world_messages-v0.2.0

[0.1.0]: https://github.com/gtker/wow_messages/releases/tag/wow_world_messages-v0.1.0
