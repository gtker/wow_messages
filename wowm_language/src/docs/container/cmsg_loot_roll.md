## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_LOOT_ROLL = 0x02A0 {
    Guid item_guid;    
    u32 item_slot;    
    RollVote vote;    
}

```
