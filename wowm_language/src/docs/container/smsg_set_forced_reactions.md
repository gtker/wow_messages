## Client Version 1.12

```rust,ignore
smsg SMSG_SET_FORCED_REACTIONS = 0x2A5 {
    u32 amount_of_reactions;    
    ForcedReaction[amount_of_reactions] reactions;    
}

```
