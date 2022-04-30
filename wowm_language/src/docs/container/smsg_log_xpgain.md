## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_LOG_XPGAIN = 0x01D0 {
    Guid target_guid;    
    u32 total_exp;    
    ExperienceAwardType exp_type;    
    if (exp_type == NON_KILL) {        
        u32 experience_without_rested;        
        f32 exp_group_bonus;        
    }    
}

```
