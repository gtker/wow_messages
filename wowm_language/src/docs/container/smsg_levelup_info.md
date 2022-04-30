## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LEVELUP_INFO = 0x01D4 {
    u32 new_level;    
    u32 health;    
    u32 mana;    
    u32 rage;    
    u32 focus;    
    u32 energy;    
    u32 happiness;    
    u32 strength;    
    u32 agility;    
    u32 stamina;    
    u32 intellect;    
    u32 spirit;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
