## Client Version 1.12

### Comment

cmangos/vmangos/mangoszero returns guid 0 and unknown 0 when talents can not be reset
cmangos/vmangos/mangoszero casts spell 14876 when resetting

### Wowm Representation
```rust,ignore
smsg MSG_TALENT_WIPE_CONFIRM_Server = 0x02AA {
    Guid wiping_npc;
    u32 cost_in_copper;
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
| 0x04 | 8 / Little | Guid | wiping_npc |  |
| 0x0C | 4 / Little | u32 | cost_in_copper |  |
