# SMSG_RAID_GROUP_ONLY

## Client Version 1.12

### Comment

used when player leaves raid group inside instance

### Wowm Representation
```rust,ignore
smsg SMSG_RAID_GROUP_ONLY = 0x0286 {
    u32 homebind_timer;
    RaidGroupError error;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | homebind_timer |  |  |
| 0x08 | ? / - | [RaidGroupError](raidgrouperror.md) | error |  |  |

