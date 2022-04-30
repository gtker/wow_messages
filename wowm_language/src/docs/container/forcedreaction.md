## Client Version 1.12

### Wowm Representation
```rust,ignore
struct ForcedReaction {
    u32 faction_id;    
    u32 reputation_rank;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | faction_id |  |
| 0x04 | 4 / Little | u32 | reputation_rank |  |
