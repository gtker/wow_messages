## Client Version 1.12

```rust,ignore
smsg SMSG_LOOT_ROLL_WON = 0x29F {
    Guid looted_target_guid;    
    u32 loot_slot;    
    u32 item_id;    
    u32 item_random_suffix;    
    u32 item_random_property_id;    
    Guid winning_player_guid;    
    u8 winning_roll;    
    RollVote vote;    
}

```
