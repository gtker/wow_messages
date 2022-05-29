## Client Version 1.12

### Description

Message to the client that is has successfully logged into the world and that it should load the map and coordinates.

### Comment

The positions and orientations do not matter since they can be overwritten in the [SMSG_UPDATE_OBJECT](./smsg_update_object.md), but the map determines which map the client loads and this is not changeable in [SMSG_UPDATE_OBJECT](./smsg_update_object.md).

### Wowm Representation
```rust,ignore
smsg SMSG_LOGIN_VERIFY_WORLD = 0x0236 {
    Map map;
    f32 position_x;
    f32 position_y;
    f32 position_z;
    f32 orientation;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | ? / - | [Map](map.md) | map |  |  |
| - | 4 / Little | f32 | position_x |  |  |
| - | 4 / Little | f32 | position_y |  |  |
| - | 4 / Little | f32 | position_z |  |  |
| - | 4 / Little | f32 | orientation |  |  |
### Examples
```c
0, 22, // size
54, 2, // opcode (566)
0, 0, 0, 0, // map: Map EASTERN_KINGDOMS (0)
205, 215, 11, 198, // position_x: f32
53, 126, 4, 195, // position_y: f32
249, 15, 167, 66, // position_z: f32
0, 0, 0, 0, // orientation: f32
```
