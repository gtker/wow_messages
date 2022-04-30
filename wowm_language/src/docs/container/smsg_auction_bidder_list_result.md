## Client Version 1.12

```rust,ignore
smsg SMSG_AUCTION_BIDDER_LIST_RESULT = 0x265 {
    u32 count;    
    AuctionListItem[count] auctions;    
    u32 total_amount_of_auctions;    
}

```
