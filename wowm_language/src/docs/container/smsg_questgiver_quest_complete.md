## Client Version 1.12

```rust,ignore
smsg SMSG_QUESTGIVER_QUEST_COMPLETE = 0x191 {
    u32 quest_id;    
    u32 unknown;    
    u32 experience_reward;    
    u32 money_reward;    
    u32 amount_of_item_rewards;    
    QuestItemReward[amount_of_item_rewards] item_rewards;    
}

```
