## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUESTGIVER_QUEST_DETAILS = 0x0188 {
    Guid guid;    
    u32 quest_id;    
    CString title;    
    CString details;    
    CString objectives;    
    u32 auto_finish;    
    u32 amount_of_choice_item_rewards;    
    QuestItemReward[amount_of_choice_item_rewards] choice_item_rewards;    
    u32 amount_of_item_rewards;    
    QuestItemReward[amount_of_item_rewards] item_rewards;    
    u32 money_reward;    
    u32 reward_spell;    
    u32 amount_of_emotes;    
    QuestDetailsEmote[amount_of_emotes] emotes;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
