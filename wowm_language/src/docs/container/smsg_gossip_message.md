## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GOSSIP_MESSAGE = 0x017D {
    Guid guid;    
    u32 title_text_id;    
    u32 amount_of_gossip_items;    
    GossipItem[amount_of_gossip_items] gossips;    
    u32 amount_of_quests;    
    QuestItem[amount_of_quests] quests;    
}

```
