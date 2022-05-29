## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_WORLD_TELEPORT = 0x0008 {
    u64 time_in_msec;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | u64 | time_in_msec |  |  |
| 0x0E | ? / - | [Map](map.md) | map |  |  |
| - | 4 / Little | f32 | position_x |  |  |
| - | 4 / Little | f32 | position_y |  |  |
| - | 4 / Little | f32 | position_z |  |  |
| - | 4 / Little | f32 | orientation |  |  |

### Examples
```c
0, 32, // size
8, 0, 0, 0, // opcode (8)
239, 190, 173, 222, 222, 202, 250, 0, // time_in_msec: u64
1, 0, 0, 0, // map: Map KALIMDOR (1)
0, 0, 128, 63, // position_x: f32
0, 0, 0, 64, // position_y: f32
0, 0, 64, 64, // position_z: f32
0, 0, 128, 64, // orientation: f32
```
