## Client Version 1.12

### Comment

vmangos/mangoszero: write in client console: setrawpos x y z o. For now, it is implemented like worldport but on the same map. Consider using MSG_MOVE_SET_RAW_POSITION_ACK.

### Wowm Representation
```rust,ignore
cmsg CMSG_MOVE_SET_RAW_POSITION = 0x00E1 {
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
| 0x06 | 4 / Little | f32 | position_x |  |  |
| 0x0A | 4 / Little | f32 | position_y |  |  |
| 0x0E | 4 / Little | f32 | position_z |  |  |
| 0x12 | 4 / Little | f32 | orientation |  |  |
