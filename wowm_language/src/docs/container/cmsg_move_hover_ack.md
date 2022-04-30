## Client Version 1.12

```rust,ignore
cmsg CMSG_MOVE_HOVER_ACK = 0xF6 {
    Guid guid;    
    u32 counter;    
    MovementInfo movement_info;    
    u32 is_applied;    
}

```
