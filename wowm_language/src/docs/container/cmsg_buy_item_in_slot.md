## Client Version 1.12

## Wowm Representation
```rust,ignore
cmsg CMSG_BUY_ITEM_IN_SLOT = 0x01A3 {
    Guid vendor_guid;    
    u32 item_id;    
    Guid bag_guid;    
    u8 bag_slot;    
    u8 amount;    
}

```
