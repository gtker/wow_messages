## Client Version 1.12

```rust,ignore
smsg SMSG_TRADE_STATUS = 0x120 {
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
