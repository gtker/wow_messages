## Client Version 1.12

```rust,ignore
smsg SMSG_ITEM_PUSH_RESULT = 0x166 {
    Guid guid;    
    NewItemSource source;    
    NewItemCreationType creation_type;    
    NewItemChatAlert alert_chat;    
    u8 bag_slot;    
    u32 item_slot;    
    u32 item_id;    
    u32 item_suffix_factor;    
    u32 item_random_property_id;    
    u32 item_count;    
}

```
