## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_NEW_WORLD = 0x003E {
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    f32 orientation;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
