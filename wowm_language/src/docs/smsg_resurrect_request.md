# SMSG_RESURRECT_REQUEST

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_RESURRECT_REQUEST = 0x015B {
    Guid guid;
    u32 name_length;
    CString name;
    u8 caster_is_spirit_healer;
    u8 respect_resurrection_timer;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x0C | 4 / Little | u32 | name_length |  |  |
| 0x10 | - / - | CString | name |  |  |
| - | 1 / - | u8 | caster_is_spirit_healer |  |  |
| - | 1 / - | u8 | respect_resurrection_timer |  |  |

