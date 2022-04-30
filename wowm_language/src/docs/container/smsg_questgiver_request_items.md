## Client Version 1.12

```rust,ignore
smsg SMSG_QUESTGIVER_REQUEST_ITEMS = 0x018B {
    Guid npc;    
    u32 quest_id;    
    CString title;    
    CString request_items_text;    
    u32 emote_delay;    
    u32 emote;    
    u32 auto_finish;    
    u32 required_money;    
    u32 amount_of_required_items;    
    QuestItemRequirement[amount_of_required_items] required_items;    
    u32 unknown1;    
    QuestCompletable completable;    
    u32 flags2;    
    u32 flags3;    
}

```
