## Client Version 1.12

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
