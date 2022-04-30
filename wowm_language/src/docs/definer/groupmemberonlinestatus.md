## Client Version 1.12

## Wowm Representation
```rust,ignore
flag GroupMemberOnlineStatus : u8 {
    OFFLINE = 0x0000;    
    ONLINE = 0x0001;    
    PVP = 0x0002;    
    DEAD = 0x0004;    
    GHOST = 0x0008;    
    PVP_FFA = 0x0010;    
    ZONE_OUT = 0x0020;    
    AFK = 0x0040;    
    DND = 0x0080;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| OFFLINE | 0x0000 | 0 | 0x0 |  |  |
| ONLINE | 0x0001 | 1 | 0x1 |  | Lua_UnitIsConnected |
| PVP | 0x0002 | 2 | 0x2 |  | Lua_UnitIsPVP |
| DEAD | 0x0004 | 4 | 0x4 |  | Lua_UnitIsDead |
| GHOST | 0x0008 | 8 | 0x8 |  | Lua_UnitIsGhost |
| PVP_FFA | 0x0010 | 16 | 0x10 |  | Lua_UnitIsPVPFreeForAll |
| ZONE_OUT | 0x0020 | 32 | 0x20 |  | used in calls from Lua_GetPlayerMapPosition/Lua_GetBattlefieldFlagPosition |
| AFK | 0x0040 | 64 | 0x40 |  | Lua_UnitIsAFK |
| DND | 0x0080 | 128 | 0x80 |  | Lua_UnitIsDND |
