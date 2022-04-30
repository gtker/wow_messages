## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_DESTROYITEM = 0x0111 {
    u8 bag;    
    u8 slot;    
    u8 amount;    
    u8 unknown1;    
    u8 unknown2;    
    u8 unknown3;    
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
| 0x06 | 1 / - | u8 | bag |  |
| 0x07 | 1 / - | u8 | slot |  |
| 0x08 | 1 / - | u8 | amount |  |
| 0x09 | 1 / - | u8 | unknown1 |  |
| 0x0A | 1 / - | u8 | unknown2 |  |
| 0x0B | 1 / - | u8 | unknown3 |  |
