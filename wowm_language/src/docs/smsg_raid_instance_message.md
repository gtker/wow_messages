## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_RAID_INSTANCE_MESSAGE = 0x02FA {
    RaidInstanceMessage message_type;
    Map map;
    u32 time_left;
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
| 0x04 | ? / - | [RaidInstanceMessage](raidinstancemessage.md) | message_type |  |
| - | ? / - | [Map](map.md) | map |  |
| - | 4 / Little | u32 | time_left |  |
