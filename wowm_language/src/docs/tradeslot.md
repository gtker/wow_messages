## Client Version 1.12

### Wowm Representation
```rust,ignore
struct TradeSlot {
    u8 trade_slot_number;
    u32 item_id;
    u32 display_id;
    u32 stack_count;
    u32 is_wrapped;
    Guid gift_wrapper;
    u32 enchantment;
    Guid item_creator;
    u32 spell_charges;
    u32 item_suffix_factor;
    u32 item_random_properties_id;
    u32 lock_id;
    u32 max_durability;
    u32 durability;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | trade_slot_number |  |
| 0x01 | 4 / Little | u32 | item_id |  |
| 0x05 | 4 / Little | u32 | display_id |  |
| 0x09 | 4 / Little | u32 | stack_count |  |
| 0x0D | 4 / Little | u32 | is_wrapped |  |
| 0x11 | 8 / Little | [Guid](../spec/packed-guid.md) | gift_wrapper |  |
| 0x19 | 4 / Little | u32 | enchantment |  |
| 0x1D | 8 / Little | [Guid](../spec/packed-guid.md) | item_creator |  |
| 0x25 | 4 / Little | u32 | spell_charges |  |
| 0x29 | 4 / Little | u32 | item_suffix_factor |  |
| 0x2D | 4 / Little | u32 | item_random_properties_id |  |
| 0x31 | 4 / Little | u32 | lock_id |  |
| 0x35 | 4 / Little | u32 | max_durability |  |
| 0x39 | 4 / Little | u32 | durability |  |
