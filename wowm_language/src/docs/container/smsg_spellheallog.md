## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SPELLHEALLOG = 0x0150 {
    PackedGuid victim_guid;    
    PackedGuid caster_guid;    
    u32 id;    
    u32 damage;    
    u8 critical;    
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
| 0x04 | - / - | PackedGuid | victim_guid |  |
| - | - / - | PackedGuid | caster_guid |  |
| - | 4 / Little | u32 | id |  |
| - | 4 / Little | u32 | damage |  |
| - | 1 / - | u8 | critical |  |
