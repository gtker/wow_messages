## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOOT_MASTER_LIST = 0x02A4 {
    u8 amount_of_players;    
    Guid[amount_of_players] guids;    
}

```
