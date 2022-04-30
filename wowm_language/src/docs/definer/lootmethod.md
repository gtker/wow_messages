## Client Version 1.12

### Wowm Representation
```rust,ignore
enum LootMethod : u8 {
    CORPSE = 1;    
    PICKPOCKETING = 2;    
    FISHING = 3;    
    DISENCHANTING = 4;    
    SKINNING = 6;    
    FISHINGHOLE = 20;    
    FISHING_FAIL = 21;    
    INSIGNIA = 22;    
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `CORPSE` | 1 (0x01) |  |  |
| `PICKPOCKETING` | 2 (0x02) |  |  |
| `FISHING` | 3 (0x03) |  |  |
| `DISENCHANTING` | 4 (0x04) |  |  |
| `SKINNING` | 6 (0x06) |  | unsupported by client, send LOOT_PICKPOCKETING instead |
| `FISHINGHOLE` | 20 (0x14) |  | unsupported by client, send LOOT_FISHING instead |
| `FISHING_FAIL` | 21 (0x15) |  | unsupported by client, send LOOT_FISHING instead |
| `INSIGNIA` | 22 (0x16) |  | unsupported by client, send LOOT_CORPSE instead |
