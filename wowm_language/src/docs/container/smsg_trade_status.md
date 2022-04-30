## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_TRADE_STATUS = 0x0120 {
    TradeStatus status;    
    if (status == BEGIN_TRADE) {        
        Guid unknown1;        
    }    
    else if (status == CLOSE_WINDOW) {        
        InventoryResult inventory_result;        
        u8 target_error;        
        u32 item_limit_category_id;        
    }    
    else if (status == ONLY_CONJURED        
        || status == NOT_ON_TAPLIST) {        
        u8 slot;        
    }    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
