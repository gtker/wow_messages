## Client Version 1.12

### Wowm Representation
```rust,ignore
struct ItemStat {
    u32 item_stat_type;    
    u32 item_stat_value;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | item_stat_type |  |
| 0x04 | 4 / Little | u32 | item_stat_value |  |
