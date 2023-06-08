# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

* BREAKING: `set_player_field_inv` method on `UpdatePlayer` and `UpdatePlayerBuilder` for vanilla that takes
  an `ItemSlot`. This replaces the previously named functions for specific positions.
* BREAKING: Additional fields for vanilla `ItemSlot`.
* `From` conversion for update mask builder types to proper types.
* `From` conversion for update mask types to `UpdateMask` type.
* `Guid::from_u32s` and `Guid::to_u32s` to create a Guid from high/low `u32`s.
* `Octal`, `Binary`, `UpperHex`, `LowerHex` for `Guid`.
* `std::fmt::Display` for new enums.

### Changed

* BREAKING: `MSG_AUCTION_HELLO_Server`, `SMSG_AUCTION_BIDDER_NOTIFICATION`, and `ReceivedMail` have had the
  variable `auction_house_id` of type `u32` changed to `auction_house` of type `AuctionHouse` enum.
* BREAKING: Vanilla `UpdatePlayer` methods `VISIBLE_ITEM_*` have been replaced by a single `VISIBLE_ITEM` method.
* BREAKING: `CMSG_TEXT_EMOTE` and `SMSG_TEXT_EMOTE` field `emote` changed from `Emote` to `u32`.

### Removed

* `write_packed_guid_into_vec`, `size`, `read_packed` methods for `Guid`.
  These were only intended to be used by `wow_world_messages` for reading `Guid`s on the wire and should provide no
  functionality for users for either `wow_world_messages` or `wow_world_base`.

## [0.1.0] - 2023-05-20

### Added

* First release.

<!-- next-url -->

[Unreleased]: https://github.com/gtker/wow_messages/compare/wow_world_messages-v0.1.1...HEAD

[0.1.0]: https://github.com/gtker/wow_messages/releases/tag/wow_world_messages-v0.1.0
