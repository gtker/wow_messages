## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GOSSIP_POI = 0x0224 {
    u32 flags;    
    f32 position_x;    
    f32 position_y;    
    u32 icon;    
    u32 data;    
    CString location_name;    
}

```
