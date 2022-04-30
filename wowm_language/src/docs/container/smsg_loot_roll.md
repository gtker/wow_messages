## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_LOOT_ROLL = 0x02A2 {
    Guid creature_guid;    
    u32 loot_slot;    
    Guid item_guid;    
    u32 item_id;    
    u32 item_random_suffix;    
    u32 item_random_property_id;    
    u8 roll_number;    
    RollVote vote;    
}

```
