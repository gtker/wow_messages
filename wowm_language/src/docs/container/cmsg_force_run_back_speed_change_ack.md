## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK = 0x00E5 {
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
