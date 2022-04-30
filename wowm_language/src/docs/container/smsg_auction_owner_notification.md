## Client Version 1.12

### Comment

vmangos/cmangos/mangoszero: this [message] causes on client to display: 'Your auction sold'

### Wowm Representation
```rust,ignore
smsg SMSG_AUCTION_OWNER_NOTIFICATION = 0x025F {
    u32 auction_id;    
    u32 bid;    
    u32 auction_out_bid;    
    Guid bidder;    
    u32 item_entry;    
    u32 item_random_property_id;    
}

```
