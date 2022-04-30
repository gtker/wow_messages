## Client Version 1.12

### Comment

vmangos/mangoszero: write in client console: setrawpos x y z o. For now, it is implemented like worldport but on the same map. Consider using MSG_MOVE_SET_RAW_POSITION_ACK.

### Wowm Representation
```rust,ignore
cmsg CMSG_MOVE_SET_RAW_POSITION = 0x00E1 {
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    f32 orientation;    
}

```
