## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg MSG_RAID_TARGET_UPDATE_Server = 0x0321 {
    RaidTargetUpdateType update_type;    
    if (update_type == FULL) {        
        RaidTargetUpdate[8] raid_targets;        
    }    
    else if (update_type == PARTIAL) {        
        RaidTargetUpdate raid_target;        
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
