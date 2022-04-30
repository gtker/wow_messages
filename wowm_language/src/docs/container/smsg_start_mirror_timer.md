## Client Version 1.12

```rust,ignore
smsg SMSG_START_MIRROR_TIMER = 0x01D9 {
    TimerType timer;    
    u32 time_remaining;    
    u32 duration;    
    u32 scale;    
    u8 is_frozen;    
    u32 id;    
}

```
