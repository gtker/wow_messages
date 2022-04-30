## Client Version 1.12

### Wowm Representation
```rust,ignore
struct GmSurveyQuestion {
    u32 question_id;    
    u8 answer;    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | question_id |  |
| 0x04 | 1 / - | u8 | answer |  |
