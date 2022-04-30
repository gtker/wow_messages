## Client Version 1.12

### Wowm Representation
```rust,ignore
struct BattlegroundPlayer {
    Guid player;    
    PvpRank rank;    
    u32 killing_blows;    
    u32 honorable_kills;    
    u32 deaths;    
    u32 bonus_honor;    
    u32 amount_of_extra_fields;    
    u32[amount_of_extra_fields] fields;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 8 / Little | Guid | player |  |
| 0x08 | ? / - | PvpRank | rank |  |
| - | 4 / Little | u32 | killing_blows |  |
| - | 4 / Little | u32 | honorable_kills |  |
| - | 4 / Little | u32 | deaths |  |
| - | 4 / Little | u32 | bonus_honor |  |
| - | 4 / Little | u32 | amount_of_extra_fields |  |
| - | ? / - | u32[amount_of_extra_fields] | fields |  |
