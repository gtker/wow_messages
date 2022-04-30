## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_SET_FORCED_REACTIONS = 0x02A5 {
    u32 amount_of_reactions;    
    ForcedReaction[amount_of_reactions] reactions;    
}

```
