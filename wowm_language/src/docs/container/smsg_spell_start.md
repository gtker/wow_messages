## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SPELL_START = 0x0131 {
    PackedGuid cast_item;    
    PackedGuid caster;    
    u32 spell;    
    CastFlags flags;    
    u32 timer;    
    SpellCastTargets targets;    
    if (flags & AMMO) {        
        u32 ammo_display_id;        
        u32 ammo_inventory_type;        
    }    
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
| 0x04 | - / - | PackedGuid | cast_item |  |
| - | - / - | PackedGuid | caster |  |
| - | 4 / Little | u32 | spell |  |
| - | ? / - | CastFlags | flags |  |
| - | 4 / Little | u32 | timer |  |
| - | ? / - | SpellCastTargets | targets |  |

If flags contains `AMMO`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | ammo_display_id |  |
| - | 4 / Little | u32 | ammo_inventory_type |  |
