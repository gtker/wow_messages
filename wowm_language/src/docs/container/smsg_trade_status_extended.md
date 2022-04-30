## Client Version 1.12

### Wowm Representation
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
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
