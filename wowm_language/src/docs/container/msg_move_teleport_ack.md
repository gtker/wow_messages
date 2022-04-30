## Client Version 1.12

### Wowm Representation
```rust,ignore
msg MSG_MOVE_TELEPORT_ACK = 0x00C7 {
    Guid guid;    
    u32 movement_counter;    
    u32 time_in_msecs;    
}

```
### Header
MSG have a header of either 6 bytes if they are sent from the client (CMSG), or 4 bytes if they are sent from the server (SMSG).

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 8 / Little | Guid | guid |  |
| 0x08 | 4 / Little | u32 | movement_counter |  |
| 0x0C | 4 / Little | u32 | time_in_msecs |  |
