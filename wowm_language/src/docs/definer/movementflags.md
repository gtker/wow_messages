## Client Version 1.12

## Wowm Representation
```rust,ignore
flag MovementFlags : u32 {
    NONE = 0x00000000;    
    FORWARD = 0x00000001;    
    BACKWARD = 0x00000002;    
    STRAFE_LEFT = 0x00000004;    
    STRAFE_RIGHT = 0x00000008;    
    TURN_LEFT = 0x00000010;    
    TURN_RIGHT = 0x00000020;    
    PITCH_UP = 0x00000040;    
    PITCH_DOWN = 0x00000080;    
    WALK_MODE = 0x00000100;    
    ON_TRANSPORT = 0x00000200;    
    LEVITATING = 0x00000400;    
    FIXED_Z = 0x00000800;    
    ROOT = 0x00001000;    
    JUMPING = 0x00002000;    
    FALLINGFAR = 0x00004000;    
    SWIMMING = 0x00200000;    
    SPLINE_ENABLED = 0x00400000;    
    CAN_FLY = 0x00800000;    
    FLYING = 0x01000000;    
    ONTRANSPORT = 0x02000000;    
    SPLINE_ELEVATION = 0x04000000;    
    WATERWALKING = 0x10000000;    
    SAFE_FALL = 0x20000000;    
    HOVER = 0x40000000;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0x00000000 | 0 | 0x0 |  |  |
| FORWARD | 0x00000001 | 1 | 0x1 |  |  |
| BACKWARD | 0x00000002 | 2 | 0x2 |  |  |
| STRAFE_LEFT | 0x00000004 | 4 | 0x4 |  |  |
| STRAFE_RIGHT | 0x00000008 | 8 | 0x8 |  |  |
| TURN_LEFT | 0x00000010 | 16 | 0x10 |  |  |
| TURN_RIGHT | 0x00000020 | 32 | 0x20 |  |  |
| PITCH_UP | 0x00000040 | 64 | 0x40 |  |  |
| PITCH_DOWN | 0x00000080 | 128 | 0x80 |  |  |
| WALK_MODE | 0x00000100 | 256 | 0x100 |  |  |
| ON_TRANSPORT | 0x00000200 | 512 | 0x200 |  |  |
| LEVITATING | 0x00000400 | 1024 | 0x400 |  |  |
| FIXED_Z | 0x00000800 | 2048 | 0x800 |  |  |
| ROOT | 0x00001000 | 4096 | 0x1000 |  |  |
| JUMPING | 0x00002000 | 8192 | 0x2000 |  |  |
| FALLINGFAR | 0x00004000 | 16384 | 0x4000 |  |  |
| SWIMMING | 0x00200000 | 2097152 | 0x200000 |  |  |
| SPLINE_ENABLED | 0x00400000 | 4194304 | 0x400000 |  |  |
| CAN_FLY | 0x00800000 | 8388608 | 0x800000 |  |  |
| FLYING | 0x01000000 | 16777216 | 0x1000000 |  |  |
| ONTRANSPORT | 0x02000000 | 33554432 | 0x2000000 |  |  |
| SPLINE_ELEVATION | 0x04000000 | 67108864 | 0x4000000 |  |  |
| WATERWALKING | 0x10000000 | 268435456 | 0x10000000 |  |  |
| SAFE_FALL | 0x20000000 | 536870912 | 0x20000000 |  |  |
| HOVER | 0x40000000 | 1073741824 | 0x40000000 |  |  |
