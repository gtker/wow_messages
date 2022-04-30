## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_PETITION_QUERY = 0x01C6 {
    u32 guild_guid;
    Guid petition_guid;
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
| 0x06 | 4 / Little | u32 | guild_guid |  |
| 0x0A | 8 / Little | Guid | petition_guid |  |
