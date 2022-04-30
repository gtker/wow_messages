## Client Version 1.12

```rust,ignore
smsg SMSG_LOOT_START_ROLL = 0x02A1 {
    Guid creature_guid;    
    u32 loot_slot;    
    u32 item_id;    
    u32 item_random_suffix;    
    u32 item_random_property_id;    
    u32 countdown_time;    
}

```
