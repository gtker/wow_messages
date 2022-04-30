## Client Version 1.12

### Wowm Representation
```rust,ignore
struct QuestItemRequirement {
    u32 item;    
    u32 item_count;    
    u32 item_display_id;    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | item |  |
| 0x04 | 4 / Little | u32 | item_count |  |
| 0x08 | 4 / Little | u32 | item_display_id |  |
