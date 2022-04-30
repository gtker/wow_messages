## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg MSG_RANDOM_ROLL_Server = 0x01FB {
    u32 minimum;    
    u32 maximum;    
    u32 actual_roll;    
    Guid guid;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
