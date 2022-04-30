## Client Version 1.12

### Wowm Representation
```rust,ignore
struct RaidInfo {
    Map map;
    u32 reset_time;
    u32 instance_id;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | Map | map |  |
| - | 4 / Little | u32 | reset_time |  |
| - | 4 / Little | u32 | instance_id |  |
