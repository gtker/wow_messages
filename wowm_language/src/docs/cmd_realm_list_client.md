## Protocol Version 2, Protocol Version 3, Protocol Version 8

### Wowm Representation
```rust,ignore
clogin CMD_REALM_LIST_Client = 0x10 {
    u32 padding = 0;
}
```
### Header
Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | 4 / Little | u32 | padding |  |  |

### Examples
```c
16, // opcode (16)
0, 0, 0, 0, // padding: u32
```
