## Client Version 1.12

```rust,ignore
struct Mail {
    u32 message_id;    
    MailType message_type;    
    if (message_type == NORMAL) {        
        Guid sender;        
    }    
    else if (message_type == CREATURE        
        || message_type == GAMEOBJECT) {        
        u32 sender_id;        
    }    
    else if (message_type == AUCTION) {        
        u32 auction_id;        
    }    
    CString subject;    
    u32 item_text_id;    
    u32 unknown1;    
    u32 stationery;    
    u32 item_id;    
    u32 item_enchant_id;    
    u32 item_random_property_id;    
    u32 item_suffix_factor;    
    u8 item_stack_size;    
    u32 item_spell_charges;    
    u32 max_durability;    
    u32 durability;    
    u32 money;    
    u32 cash_on_delivery_amount;    
    u32 checked_timestamp;    
    f32 expiration_time;    
    u32 mail_template_id;    
}

```
