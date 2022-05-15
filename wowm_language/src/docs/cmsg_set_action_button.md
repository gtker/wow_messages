## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_SET_ACTION_BUTTON = 0x0128 {
    u8 button;
    u32 action_type;
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
| 0x06 | 1 / - | u8 | button |  |  |
| 0x07 | 4 / Little | u32 | action_type |  | Most significant byte determines types, rest is action. |
