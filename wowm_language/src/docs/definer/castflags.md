## Client Version 1.12

## Wowm Representation
```rust,ignore
flag CastFlags : u16 {
    NONE = 0x00000000;    
    HIDDEN_COMBATLOG = 0x00000001;    
    UNKNOWN2 = 0x00000002;    
    UNKNOWN3 = 0x00000004;    
    UNKNOWN4 = 0x00000008;    
    UNKNOWN5 = 0x00000010;    
    AMMO = 0x00000020;    
    UNKNOWN7 = 0x00000040;    
    UNKNOWN8 = 0x00000080;    
    UNKNOWN9 = 0x00000100;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0x00000000 | 0 | 0x0 |  |  |
| HIDDEN_COMBATLOG | 0x00000001 | 1 | 0x1 |  | mangoszero/cmangos/vmangos: hide in combat log? |
| UNKNOWN2 | 0x00000002 | 2 | 0x2 |  |  |
| UNKNOWN3 | 0x00000004 | 4 | 0x4 |  |  |
| UNKNOWN4 | 0x00000008 | 8 | 0x8 |  |  |
| UNKNOWN5 | 0x00000010 | 16 | 0x10 |  |  |
| AMMO | 0x00000020 | 32 | 0x20 |  | cmangos/vmangos/mangoszero: Projectiles visual |
| UNKNOWN7 | 0x00000040 | 64 | 0x40 |  | cmangos/vmangos/mangoszero: !0x41 mask used to call CGTradeSkillInfo::DoRecast |
| UNKNOWN8 | 0x00000080 | 128 | 0x80 |  |  |
| UNKNOWN9 | 0x00000100 | 256 | 0x100 |  |  |
