## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SPELLNONMELEEDAMAGELOG = 0x0250 {
    PackedGuid target;    
    PackedGuid attacker;    
    u32 spell;    
    u32 damage;    
    SpellSchool school;    
    u32 absorbed_damage;    
    u32 resisted;    
    u8 periodic_log;    
    u8 unused;    
    u32 blocked;    
    u32 hit_info;    
    u8 extend_flag;    
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
| 0x04 | - / - | PackedGuid | target |  |
| - | - / - | PackedGuid | attacker |  |
| - | 4 / Little | u32 | spell |  |
| - | 4 / Little | u32 | damage |  |
| - | ? / - | SpellSchool | school |  |
| - | 4 / Little | u32 | absorbed_damage |  |
| - | 4 / Little | u32 | resisted |  |
| - | 1 / - | u8 | periodic_log |  |
| - | 1 / - | u8 | unused |  |
| - | 4 / Little | u32 | blocked |  |
| - | 4 / Little | u32 | hit_info |  |
| - | 1 / - | u8 | extend_flag |  |
