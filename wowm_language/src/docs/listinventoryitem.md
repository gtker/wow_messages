# ListInventoryItem
## Client Version 1.12

### Wowm Representation
```rust,ignore
struct ListInventoryItem {
    u32 item_stack_count;
    u32 item_id;
    u32 item_display_id;
    u32 max_items;
    u32 price;
    u32 max_durability;
    u32 durability;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | item_stack_count |  |
| 0x04 | 4 / Little | u32 | item_id |  |
| 0x08 | 4 / Little | u32 | item_display_id |  |
| 0x0C | 4 / Little | u32 | max_items |  |
| 0x10 | 4 / Little | u32 | price |  |
| 0x14 | 4 / Little | u32 | max_durability |  |
| 0x18 | 4 / Little | u32 | durability |  |
