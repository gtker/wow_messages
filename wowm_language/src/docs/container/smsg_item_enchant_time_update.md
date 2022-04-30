## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_ITEM_ENCHANT_TIME_UPDATE = 0x01EB {
    Guid item_guid;    
    u32 slot;    
    u32 duration;    
    Guid player_guid;    
}

```
