# CMSG_FORCE_MOVE_ROOT_ACK
## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_FORCE_MOVE_ROOT_ACK = 0x00E9 {
    Guid guid;
    u32 movement_counter;
    MovementInfo movement_info;
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
| 0x0E | 4 / Little | u32 | movement_counter |  |
| 0x12 | ? / - | [MovementInfo](movementinfo.md) | movement_info |  |
