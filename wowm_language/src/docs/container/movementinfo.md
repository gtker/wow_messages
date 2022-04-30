## Client Version 1.12

## Wowm Representation
```rust,ignore
struct MovementInfo {
    MovementFlags flags;    
    u32 timestamp;    
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    f32 orientation;    
    if (flags & ON_TRANSPORT) {        
        TransportInfo transport;        
    }    
    if (flags & SWIMMING) {        
        f32 pitch;        
    }    
    f32 fall_time;    
    if (flags & JUMPING) {        
        f32 z_speed;        
        f32 cos_angle;        
        f32 sin_angle;        
        f32 xy_speed;        
    }    
    if (flags & SPLINE_ELEVATION) {        
        f32 spline_elevation;        
    }    
}

```
