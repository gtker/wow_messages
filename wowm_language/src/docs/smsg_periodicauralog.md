## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PERIODICAURALOG = 0x024E {
    PackedGuid target;
    PackedGuid caster;
    u32 spell;
    u32 amount_of_auras;
    AuraLog[amount_of_auras] auras;
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
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | target |  |  |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | caster |  |  |
| - | 4 / Little | u32 | spell |  |  |
| - | 4 / Little | u32 | amount_of_auras |  |  |
| - | ? / - | [AuraLog](auralog.md)[amount_of_auras] | auras |  |  |

