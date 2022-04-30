## Client Version 1.12

### Wowm Representation
```rust,ignore
struct SpellCooldownStatus {
    u32 id;    
    u32 cooldown_time_in_msecs;    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | id |  |
| 0x04 | 4 / Little | u32 | cooldown_time_in_msecs |  |
