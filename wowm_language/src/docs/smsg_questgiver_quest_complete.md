## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUESTGIVER_QUEST_COMPLETE = 0x0191 {
    u32 quest_id;
    u32 unknown;
    u32 experience_reward;
    u32 money_reward;
    u32 amount_of_item_rewards;
    QuestItemReward[amount_of_item_rewards] item_rewards;
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
| 0x04 | 4 / Little | u32 | quest_id |  |  |
| 0x08 | 4 / Little | u32 | unknown |  | cmangos/vmangos/mangoszero: set to 0x03 |
| 0x0C | 4 / Little | u32 | experience_reward |  |  |
| 0x10 | 4 / Little | u32 | money_reward |  |  |
| 0x14 | 4 / Little | u32 | amount_of_item_rewards |  |  |
| 0x18 | ? / - | [QuestItemReward](questitemreward.md)[amount_of_item_rewards] | item_rewards |  |  |

