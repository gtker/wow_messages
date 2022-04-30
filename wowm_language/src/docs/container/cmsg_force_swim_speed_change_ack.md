## Client Version 1.12

```rust,ignore
cmsg CMSG_FORCE_SWIM_SPEED_CHANGE_ACK = 0xE7 {
    Guid guid;    
    u32 counter;    
    MovementInfo movement_info;    
    f32 new_speed;    
}

```
