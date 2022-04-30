## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOOT_ALL_PASSED = 0x029E {
    Guid looted_target_guid;    
    u32 loot_slot;    
    u32 item_id;    
    u32 item_random_property_id;    
    u32 item_random_suffix_id;    
}

```
