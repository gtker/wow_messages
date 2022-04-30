## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_TUTORIAL_FLAGS = 0x00FD {
    u32 tutorial_data0;    
    u32 tutorial_data1;    
    u32 tutorial_data2;    
    u32 tutorial_data3;    
    u32 tutorial_data4;    
    u32 tutorial_data5;    
    u32 tutorial_data6;    
    u32 tutorial_data7;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
