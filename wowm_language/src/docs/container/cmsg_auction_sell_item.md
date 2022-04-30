## Client Version 1.12

```rust,ignore
cmsg CMSG_AUCTION_SELL_ITEM = 0x0256 {
    Guid auctioneer_guid;    
    Guid object_guid;    
    u32 stack_size;    
    u32 starting_bid;    
    u32 buyout;    
    u32 auction_duration_in_minutes;    
}

```
