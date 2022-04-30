## Client Version 1.12

```rust,ignore
smsg SMSG_QUESTGIVER_OFFER_REWARD = 0x18D {
    Guid npc;    
    u32 quest_id;    
    CString title;    
    CString offer_reward_text;    
    u32 enable_next;    
    u32 amount_of_emotes;    
    NpcTextUpdateEmote[amount_of_emotes] emotes;    
    u32 amount_of_choice_item_rewards;    
    QuestItemReward[amount_of_choice_item_rewards] choice_item_rewards;    
    u32 amount_of_item_rewards;    
    QuestItemReward[amount_of_item_rewards] item_rewards;    
    u32 money_reward;    
    u32 reward_spell;    
    u32 reward_spell_cast;    
}

```
