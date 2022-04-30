## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_WORLD_TELEPORT = 0x0008 {
    Guid time_in_msec;
    Map map;
    f32 position_x;
    f32 position_y;
    f32 position_z;
    f32 orientation;
}
```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | time_in_msec |  |
| 0x0E | ? / - | [Map](map.md) | map |  |
| - | 4 / Little | f32 | position_x |  |
| - | 4 / Little | f32 | position_y |  |
| - | 4 / Little | f32 | position_z |  |
| - | 4 / Little | f32 | orientation |  |
