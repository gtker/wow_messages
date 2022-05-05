# CMD_XFER_ACCEPT
## Protocol Version 3

### Wowm Representation
```rust,ignore
clogin CMD_XFER_ACCEPT = 0x32 {
}
```
### Header
Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
### Examples
```c
50, // opcode (50)
```
