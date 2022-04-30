## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_PET_SET_ACTION = 0x0174 {
    Guid guid;    
    u32 position1;    
    u32 data1;    
    optional extra {    
        u32 position2;        
        u32 data2;        
    }    
}

```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
