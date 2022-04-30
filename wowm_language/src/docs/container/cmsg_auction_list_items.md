## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_AUCTION_LIST_ITEMS = 0x0258 {
    Guid auctioneer_guid;    
    u32 list_start_item;    
    CString searched_name;    
    u8 minimum_level;    
    u8 maximum_level;    
    u32 auction_slot_id;    
    u32 auction_main_category;    
    u32 auction_sub_category;    
    u32 auction_quality;    
    u8 usable;    
}

```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
