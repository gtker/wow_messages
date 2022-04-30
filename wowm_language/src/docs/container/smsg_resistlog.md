## Client Version 1.12

### Comment

Structure as comment on https://github1s.com/mangoszero/server/blob/HEAD/src/game/Server/Opcodes.h#L525

### Wowm Representation
```rust,ignore
smsg SMSG_RESISTLOG = 0x01D6 {
    Guid guid1;    
    Guid guid2;    
    u32 unknown1;    
    f32 unknown2;    
    f32 unknown3;    
    u32 unknown4;    
    u32 unknown5;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
