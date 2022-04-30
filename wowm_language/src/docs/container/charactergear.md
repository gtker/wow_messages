## Client Version 1.12

### Wowm Representation
```rust,ignore
struct CharacterGear {
    u32 equipment_display_id;    
    InventoryType inventory_type;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | equipment_display_id |  |
| 0x04 | ? / - | InventoryType | inventory_type |  |
