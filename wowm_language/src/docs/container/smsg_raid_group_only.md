## Client Version 1.12

### Comment

used when player leaves raid group inside instance

### Wowm Representation
```rust,ignore
smsg SMSG_RAID_GROUP_ONLY = 0x0286 {
    u32 homebind_timer;    
    RaidGroupError error;    
}

```
