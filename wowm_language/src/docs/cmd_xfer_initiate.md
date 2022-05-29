## Protocol Version 3

### Wowm Representation
```rust,ignore
slogin CMD_XFER_INITIATE = 0x30 {
}
```
### Header
Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

This message has no fields in the body.

### Examples
```c
48, // opcode (48)
```
