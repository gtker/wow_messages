## Client Version 1.12

```rust,ignore
smsg SMSG_QUESTGIVER_QUEST_LIST = 0x185 {
    Guid npc;    
    CString title;    
    u32 emote_delay;    
    u32 emote;    
    u8 amount_of_entries;    
    QuestItem[amount_of_entries] quest_items;    
}

```
