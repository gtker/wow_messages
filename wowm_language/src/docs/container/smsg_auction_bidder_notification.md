## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_AUCTION_BIDDER_NOTIFICATION = 0x025E {
    u32 auction_house_id;    
    u32 auction_id;    
    Guid bidder;    
    u32 won;    
    u32 out_bid;    
    u32 item_template;    
    u32 item_random_property_id;    
}

```
