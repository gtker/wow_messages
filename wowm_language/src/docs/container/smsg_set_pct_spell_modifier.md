## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SET_PCT_SPELL_MODIFIER = 0x0267 {
    u8 eff;    
    u8 op;    
    u32 value;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 1 / - | u8 | eff |  |
| 0x05 | 1 / - | u8 | op |  |
| 0x06 | 4 / Little | u32 | value |  |
