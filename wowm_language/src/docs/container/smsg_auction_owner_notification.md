## Client Version 1.12

### Comment

vmangos/cmangos/mangoszero: this [message] causes on client to display: 'Your auction sold'

### Wowm Representation
```rust,ignore
smsg SMSG_AUCTION_OWNER_NOTIFICATION = 0x025F {
    u32 auction_id;
    u32 bid;
    u32 auction_out_bid;
    Guid bidder;
    u32 item_entry;
    u32 item_random_property_id;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 4 / Little | u32 | auction_id |  |
| 0x08 | 4 / Little | u32 | bid |  |
| 0x0C | 4 / Little | u32 | auction_out_bid |  |
| 0x10 | 8 / Little | Guid | bidder |  |
| 0x18 | 4 / Little | u32 | item_entry |  |
| 0x1C | 4 / Little | u32 | item_random_property_id |  |
