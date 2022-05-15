## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_TRAINER_LIST = 0x01B1 {
    Guid guid;
    u32 trainer_type;
    u32 amount_of_spells;
    TrainerSpell[amount_of_spells] spells;
    CString greeting;
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
| 0x0C | 4 / Little | u32 | trainer_type |  |  |
| 0x10 | 4 / Little | u32 | amount_of_spells |  |  |
| 0x14 | ? / - | [TrainerSpell](trainerspell.md)[amount_of_spells] | spells |  |  |
| - | - / - | CString | greeting |  |  |
