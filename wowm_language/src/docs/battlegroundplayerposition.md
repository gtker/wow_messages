## Client Version 1.12

### Wowm Representation
```rust,ignore
struct BattlegroundPlayerPosition {
    Guid player;
    f32 position_x;
    f32 position_y;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | player |  |  |
| 0x08 | 4 / Little | f32 | position_x |  |  |
| 0x0C | 4 / Little | f32 | position_y |  |  |
