## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_ENVIRONMENTALDAMAGELOG = 0x01FC {
    Guid guid;
    EnvironmentalDamageType damage_type;
    u32 damage;
    u32 absorb;
    u32 resist;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
| 0x0C | ? / - | [EnvironmentalDamageType](environmentaldamagetype.md) | damage_type |  |
| - | 4 / Little | u32 | damage |  |
| - | 4 / Little | u32 | absorb |  |
| - | 4 / Little | u32 | resist |  |
