# CMSG_AUCTION_SELL_ITEM

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm#L3).
```rust,ignore
cmsg CMSG_AUCTION_SELL_ITEM = 0x0256 {
    Guid auctioneer_guid;
    Guid object_guid;
    u32 stack_size;
    u32 starting_bid;
    u32 buyout;
    u32 auction_duration_in_minutes;
}
```
### Header

CMSG have a header of 6 bytes.

#### CMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | auctioneer_guid |  |  |
| 0x0E | 8 / Little | [Guid](../spec/packed-guid.md) | object_guid |  |  |
| 0x16 | 4 / Little | u32 | stack_size |  |  |
| 0x1A | 4 / Little | u32 | starting_bid |  |  |
| 0x1E | 4 / Little | u32 | buyout |  |  |
| 0x22 | 4 / Little | u32 | auction_duration_in_minutes |  |  |

