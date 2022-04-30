## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK = 0x02DD {
    Guid guid;
    u32 counter;
    MovementInfo movement_info;
    f32 new_speed;
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
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
| 0x0E | 4 / Little | u32 | counter |  |
| 0x12 | ? / - | [MovementInfo](movementinfo.md) | movement_info |  |
| - | 4 / Little | f32 | new_speed |  |
