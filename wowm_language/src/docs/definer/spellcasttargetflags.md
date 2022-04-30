## Client Version 1.12

## Wowm Representation
```rust,ignore
flag SpellCastTargetFlags : u16 {
    SELF = 0x00000000;    
    UNUSED1 = 0x00000001;    
    UNIT = 0x00000002;    
    UNIT_RAID = 0x00000004;    
    UNIT_PARTY = 0x00000008;    
    ITEM = 0x00000010;    
    SOURCE_LOCATION = 0x00000020;    
    DEST_LOCATION = 0x00000040;    
    UNIT_ENEMY = 0x00000080;    
    UNIT_ALLY = 0x00000100;    
    CORPSE_ENEMY = 0x00000200;    
    UNIT_DEAD = 0x00000400;    
    GAMEOBJECT = 0x00000800;    
    TRADE_ITEM = 0x00001000;    
    STRING = 0x00002000;    
    LOCKED = 0x00004000;    
    CORPSE_ALLY = 0x00008000;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SELF | 0x00000000 | 0 | 0x0 |  |  |
| UNUSED1 | 0x00000001 | 1 | 0x1 |  | not used in any spells as of 2.4.3 (can be set dynamically) |
| UNIT | 0x00000002 | 2 | 0x2 |  | pguid |
| UNIT_RAID | 0x00000004 | 4 | 0x4 |  | not used in any spells as of 2.4.3 (can be set dynamically) - raid member |
| UNIT_PARTY | 0x00000008 | 8 | 0x8 |  | not used in any spells as of 2.4.3 (can be set dynamically) - party member |
| ITEM | 0x00000010 | 16 | 0x10 |  | pguid |
| SOURCE_LOCATION | 0x00000020 | 32 | 0x20 |  | 3xfloat |
| DEST_LOCATION | 0x00000040 | 64 | 0x40 |  | 3xfloat |
| UNIT_ENEMY | 0x00000080 | 128 | 0x80 |  | CanAttack == true |
| UNIT_ALLY | 0x00000100 | 256 | 0x100 |  | CanAssist == true |
| CORPSE_ENEMY | 0x00000200 | 512 | 0x200 |  | pguid, CanAssist == false |
| UNIT_DEAD | 0x00000400 | 1024 | 0x400 |  | skinning-like effects |
| GAMEOBJECT | 0x00000800 | 2048 | 0x800 |  | pguid, 0 spells in 2.4.3 |
| TRADE_ITEM | 0x00001000 | 4096 | 0x1000 |  | pguid, 0 spells |
| STRING | 0x00002000 | 8192 | 0x2000 |  | string, 0 spells |
| LOCKED | 0x00004000 | 16384 | 0x4000 |  | 199 spells, opening object/lock |
| CORPSE_ALLY | 0x00008000 | 32768 | 0x8000 |  | pguid, CanAssist == true |
