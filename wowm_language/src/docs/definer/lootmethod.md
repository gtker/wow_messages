## Client Version 1.12

## Wowm Representation
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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| CORPSE | 1 | 1 | 0x1 |  |  |
| PICKPOCKETING | 2 | 2 | 0x2 |  |  |
| FISHING | 3 | 3 | 0x3 |  |  |
| DISENCHANTING | 4 | 4 | 0x4 |  |  |
| SKINNING | 6 | 6 | 0x6 |  | unsupported by client, send LOOT_PICKPOCKETING instead |
| FISHINGHOLE | 20 | 20 | 0x14 |  | unsupported by client, send LOOT_FISHING instead |
| FISHING_FAIL | 21 | 21 | 0x15 |  | unsupported by client, send LOOT_FISHING instead |
| INSIGNIA | 22 | 22 | 0x16 |  | unsupported by client, send LOOT_CORPSE instead |
