## Client Version 1.12

### Comment

cmangos and vmangos/mangoszero disagree about packed and extra u8

### Wowm Representation
```rust,ignore
smsg SMSG_ENCHANTMENTLOG = 0x01D7 {
    Guid target_guid;    
    Guid caster_guid;    
    u32 item;    
    u32 spell;    
    u8 unknown1;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
