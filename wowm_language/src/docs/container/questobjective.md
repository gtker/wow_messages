## Client Version 1.12

### Wowm Representation
```rust,ignore
struct QuestObjective {
    u32 creature_id;    
    u32 kill_count;    
    u32 required_item_id;    
    u32 required_item_count;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | creature_id |  |
| 0x04 | 4 / Little | u32 | kill_count |  |
| 0x08 | 4 / Little | u32 | required_item_id |  |
| 0x0C | 4 / Little | u32 | required_item_count |  |
