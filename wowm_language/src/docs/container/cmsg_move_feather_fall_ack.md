## Client Version 1.12

```rust,ignore
cmsg CMSG_MOVE_FEATHER_FALL_ACK = 0x2CF {
    Guid guid;    
    u32 movement_counter;    
    MovementInfo movement_info;    
    u32 apply;    
}

```
