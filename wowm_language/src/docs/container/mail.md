## Client Version 1.12

### Wowm Representation
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
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | message_id |  |
| 0x04 | ? / - | MailType | message_type |  |

If message_type is equal to `NORMAL`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 8 / Little | Guid | sender |  |

Else If message_type is equal to `CREATURE` **or** 
is equal to `GAMEOBJECT`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | sender_id |  |

Else If message_type is equal to `AUCTION`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | auction_id |  |
| - | - / - | CString | subject |  |
| - | 4 / Little | u32 | item_text_id |  |
| - | 4 / Little | u32 | unknown1 |  |
| - | 4 / Little | u32 | stationery |  |
| - | 4 / Little | u32 | item_id |  |
| - | 4 / Little | u32 | item_enchant_id |  |
| - | 4 / Little | u32 | item_random_property_id |  |
| - | 4 / Little | u32 | item_suffix_factor |  |
| - | 1 / - | u8 | item_stack_size |  |
| - | 4 / Little | u32 | item_spell_charges |  |
| - | 4 / Little | u32 | max_durability |  |
| - | 4 / Little | u32 | durability |  |
| - | 4 / Little | u32 | money |  |
| - | 4 / Little | u32 | cash_on_delivery_amount |  |
| - | 4 / Little | u32 | checked_timestamp |  |
| - | 4 / Little | f32 | expiration_time |  |
| - | 4 / Little | u32 | mail_template_id |  |
