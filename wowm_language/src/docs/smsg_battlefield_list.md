## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_BATTLEFIELD_LIST = 0x023D {
    Guid battlemaster;
    Map map;
    u8 unknown1;
    u32 unknown2;
    u8 unknown3;
    u32 number_of_battlegrounds;
    u32[number_of_battlegrounds] battlegrounds;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | battlemaster |  |
| 0x0C | ? / - | [Map](map.md) | map |  |
| - | 1 / - | u8 | unknown1 |  |
| - | 4 / Little | u32 | unknown2 |  |
| - | 1 / - | u8 | unknown3 |  |
| - | 4 / Little | u32 | number_of_battlegrounds |  |
| - | ? / - | u32[number_of_battlegrounds] | battlegrounds |  |
