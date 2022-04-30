## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_INITIAL_SPELLS = 0x012A {
    u8 unknown1;    
    u16 spell_count;    
    InitialSpell[spell_count] initial_spells;    
    u16 cooldown_count;    
    CooldownSpell[cooldown_count] cooldowns;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
