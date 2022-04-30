## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_AUCTION_LIST_RESULT = 0x025C {
    u32 count;    
    AuctionListItem[count] auctions;    
    u32 total_amount_of_auctions;    
}

```
