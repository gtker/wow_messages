## Client Version 1.12

```rust,ignore
smsg SMSG_AUCTION_OWNER_LIST_RESULT = 0x25D {
    u32 count;    
    AuctionListItem[count] auctions;    
    u32 total_amount_of_auctions;    
}

```
