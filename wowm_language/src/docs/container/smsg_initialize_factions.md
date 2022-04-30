## Client Version 1.12

```rust,ignore
smsg SMSG_INITIALIZE_FACTIONS = 0x122 {
    u32 amount_of_factions;    
    FactionInitializer[amount_of_factions] factions;    
}

```
