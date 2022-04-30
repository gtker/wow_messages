## Client Version 1.12

### Comment

According to cmangos: 'Default UI handler for this is bugged, args dont match. Gotta do a full update with SMSG_START_MIRROR_TIMER to avoid lua errors.

### Wowm Representation
```rust,ignore
smsg SMSG_PAUSE_MIRROR_TIMER = 0x01DA {
    TimerType timer;    
    u8 is_frozen;    
}

```
