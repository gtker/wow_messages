## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_ATTACKSTOP = 0x0144 {
    PackedGuid player;
    PackedGuid enemy;
    u32 unknown1;
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
| 0x04 | - / - | PackedGuid | player |  |
| - | - / - | PackedGuid | enemy |  |
| - | 4 / Little | u32 | unknown1 |  |
