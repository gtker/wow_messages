## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_LOOT_ROLL = 0x02A0 {
    Guid item_guid;
    u32 item_slot;
    RollVote vote;
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
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | 8 / Little | Guid | item_guid |  |
| 0x0E | 4 / Little | u32 | item_slot |  |
| 0x12 | ? / - | RollVote | vote |  |
