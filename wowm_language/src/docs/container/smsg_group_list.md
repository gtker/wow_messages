## Client Version 1.12

```rust,ignore
smsg SMSG_GROUP_LIST = 0x7D {
    GroupType group_type;    
    u8 own_flags;    
    u32 amount_of_members;    
    GroupListMember[amount_of_members] members;    
    Guid leader;    
    optional group_not_empty {    
        GroupLootSetting loot_setting;        
        Guid master_loot;        
        ItemQuality loot_threshold;        
    }    
}

```
