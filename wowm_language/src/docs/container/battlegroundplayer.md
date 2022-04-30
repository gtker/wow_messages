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
