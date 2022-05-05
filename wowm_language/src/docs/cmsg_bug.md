## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_BUG = 0x01CA {
    u32 suggestion;
    u32 content_length;
    CString content;
    u32 type_length;
    CString bug_type;
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
| 0x06 | 4 / Little | u32 | suggestion |  |
| 0x0A | 4 / Little | u32 | content_length |  |
| 0x0E | - / - | CString | content |  |
| - | 4 / Little | u32 | type_length |  |
| - | - / - | CString | bug_type |  |
