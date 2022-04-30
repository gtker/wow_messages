## Client Version 1.12

```rust,ignore
smsg SMSG_SET_FACTION_STANDING = 0x124 {
    u32 amount_of_factions;    
    Faction[amount_of_factions] factions;    
}

```
