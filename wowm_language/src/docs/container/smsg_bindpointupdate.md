## Client Version 1.12

### Description
Set new hearthstone location.

### Wowm Representation
```rust,ignore
smsg SMSG_BINDPOINTUPDATE = 0x0155 {
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    Map map;    
    Area area;    
}

```
