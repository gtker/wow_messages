## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_TRADE_STATUS_EXTENDED = 0x0121 {
    u8 self_player;    
    u32 trade_slot_count1;    
    u32 trade_slot_count2;    
    u32 money_in_trade;    
    u32 spell_on_lowest_slot;    
    TradeSlot[7] trade_slots;    
}

```
