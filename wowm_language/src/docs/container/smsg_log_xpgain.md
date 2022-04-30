## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOG_XPGAIN = 0x01D0 {
    Guid target_guid;    
    u32 total_exp;    
    ExperienceAwardType exp_type;    
    if (exp_type == NON_KILL) {        
        u32 experience_without_rested;        
        f32 exp_group_bonus;        
    }    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
