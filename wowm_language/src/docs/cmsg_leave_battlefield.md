## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_LEAVE_BATTLEFIELD = 0x02E1 {
    u8 unknown1;
    u8 battle_ground_type_id;
    u16 unknown2;
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
| 0x06 | 1 / - | u8 | unknown1 |  |
| 0x07 | 1 / - | u8 | battle_ground_type_id |  |
| 0x08 | 2 / Little | u16 | unknown2 |  |
