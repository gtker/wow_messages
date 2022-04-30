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
