# SMSG_LOOT_ROLL_WON

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOOT_ROLL_WON = 0x029F {
    Guid looted_target_guid;
    u32 loot_slot;
    u32 item_id;
    u32 item_random_suffix;
    u32 item_random_property_id;
    Guid winning_player_guid;
    u8 winning_roll;
    RollVote vote;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | looted_target_guid |  |  |
| 0x0C | 4 / Little | u32 | loot_slot |  |  |
| 0x10 | 4 / Little | u32 | item_id |  |  |
| 0x14 | 4 / Little | u32 | item_random_suffix |  | vmangos/mangoszero: not used ? |
| 0x18 | 4 / Little | u32 | item_random_property_id |  |  |
| 0x1C | 8 / Little | [Guid](../spec/packed-guid.md) | winning_player_guid |  |  |
| 0x24 | 1 / - | u8 | winning_roll |  | rollnumber related to SMSG_LOOT_ROLL |
| 0x25 | ? / - | [RollVote](rollvote.md) | vote |  | Rolltype related to SMSG_LOOT_ROLL |

