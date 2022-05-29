## Client Version 1.12

### Wowm Representation
```rust,ignore
struct TransportInfo {
    PackedGuid guid;
    f32 position_x;
    f32 position_y;
    f32 position_z;
    f32 orientation;
    u32 timestamp;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |  |
| - | 4 / Little | f32 | position_x |  |  |
| - | 4 / Little | f32 | position_y |  |  |
| - | 4 / Little | f32 | position_z |  |  |
| - | 4 / Little | f32 | orientation |  |  |
| - | 4 / Little | u32 | timestamp |  |  |

