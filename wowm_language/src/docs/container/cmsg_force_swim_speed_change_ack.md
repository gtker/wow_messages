## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_FORCE_SWIM_SPEED_CHANGE_ACK = 0x00E7 {
    Guid guid;    
    u32 counter;    
    MovementInfo movement_info;    
    f32 new_speed;    
}

```
