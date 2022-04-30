## Client Version 1.12

### Wowm Representation
```rust,ignore
struct QuestItemReward {
    u32 item;
    u32 item_count;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | item |  |
| 0x04 | 4 / Little | u32 | item_count |  |
