## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUEST_QUERY_RESPONSE = 0x005D {
    u32 quest_id;    
    u32 quest_method;    
    u32 quest_level;    
    u32 zone_or_sort;    
    u32 quest_type;    
    u32 reputation_objective_faction;    
    u32 reputation_objective_value;    
    u32 required_opposite_faction;    
    u32 required_opposite_reputation_value;    
    u32 next_quest_in_chain;    
    u32 money_reward;    
    u32 max_level_money_reward;    
    u32 reward_spell;    
    u32 source_item_id;    
    u32 quest_flags;    
    QuestItemReward[4] rewards;    
    QuestItemReward[6] choice_rewards;    
    u32 point_map_id;    
    f32 position_x;    
    f32 position_y;    
    u32 point_opt;    
    CString title;    
    CString objective_text;    
    CString details;    
    CString end_text;    
    QuestObjective[4] objectives;    
    CString[4] objective_texts;    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
