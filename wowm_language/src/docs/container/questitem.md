## Client Version 1.12

### Wowm Representation
```rust,ignore
struct QuestItem {
    u32 quest_id;    
    u32 quest_icon;    
    u32 level;    
    CString title;    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | quest_id |  |
| 0x04 | 4 / Little | u32 | quest_icon |  |
| 0x08 | 4 / Little | u32 | level |  |
| 0x0C | - / - | CString | title |  |
