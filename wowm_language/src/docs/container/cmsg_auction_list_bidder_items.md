## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_AUCTION_LIST_BIDDER_ITEMS = 0x0264 {
    Guid auctioneer;    
    u32 start_from_page;    
    u32 amount_of_outbidded_items;    
    u32[amount_of_outbidded_items] outbid_item_ids;    
}

```
