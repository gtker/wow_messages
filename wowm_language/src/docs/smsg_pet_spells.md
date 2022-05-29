## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PET_SPELLS = 0x0179 {
    Guid pet;
    u32 unknown1;
    PetReactState react;
    PetCommandState command;
    u16 unknown2;
    u32[10] action_bars;
    u8 amount_of_spells;
    u32[amount_of_spells] spells;
    u8 amount_of_cooldowns;
    PetSpellCooldown[amount_of_cooldowns] cooldowns;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | pet |  |  |
| 0x0C | 4 / Little | u32 | unknown1 |  | mangoszero: set to 0 |
| 0x10 | ? / - | [PetReactState](petreactstate.md) | react |  |  |
| - | ? / - | [PetCommandState](petcommandstate.md) | command |  |  |
| - | 2 / Little | u16 | unknown2 |  | mangoszero: set to 0 |
| - | ? / - | u32[10] | action_bars |  |  |
| - | 1 / - | u8 | amount_of_spells |  |  |
| - | ? / - | u32[amount_of_spells] | spells |  |  |
| - | 1 / - | u8 | amount_of_cooldowns |  |  |
| - | ? / - | [PetSpellCooldown](petspellcooldown.md)[amount_of_cooldowns] | cooldowns |  |  |

