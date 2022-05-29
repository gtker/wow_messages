## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GROUP_LIST = 0x007D {
    GroupType group_type;
    u8 own_flags;
    u32 amount_of_members;
    GroupListMember[amount_of_members] members;
    Guid leader;
    optional group_not_empty {
        GroupLootSetting loot_setting;
        Guid master_loot;
        ItemQuality loot_threshold;
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

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | ? / - | [GroupType](grouptype.md) | group_type |  |  |
| - | 1 / - | u8 | own_flags |  | mangoszero/cmangos/vmangos: own flags (groupid | (assistant?0x80:0)) |
| - | 4 / Little | u32 | amount_of_members |  |  |
| - | ? / - | [GroupListMember](grouplistmember.md)[amount_of_members] | members |  |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | leader |  |  |

Optionally the following fields can be present. This can only be detected by looking at the size of the message.

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [GroupLootSetting](grouplootsetting.md) | loot_setting |  |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | master_loot |  | Zero if loot_setting is not MASTER_LOOT |
| - | ? / - | [ItemQuality](itemquality.md) | loot_threshold |  |  |

