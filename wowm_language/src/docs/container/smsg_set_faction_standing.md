## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_SET_FACTION_STANDING = 0x0124 {
    u32 amount_of_factions;    
    Faction[amount_of_factions] factions;    
}

```
