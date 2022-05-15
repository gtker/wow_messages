## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg SMSG_UPDATE_OBJECT = 0x00A9 {
    u32 amount_of_objects;
    u8 has_transport;
    Object[amount_of_objects] objects;
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
| 0x06 | 4 / Little | u32 | amount_of_objects |  |
| 0x0A | 1 / - | u8 | has_transport |  |
| 0x0B | ? / - | [Object](object.md)[amount_of_objects] | objects |  |
