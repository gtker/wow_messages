## Client Version 1.12

## Wowm Representation
```rust,ignore
cmsg CMSG_MOVE_HOVER_ACK = 0x00F6 {
    Guid guid;    
    u32 counter;    
    MovementInfo movement_info;    
    u32 is_applied;    
}

```
