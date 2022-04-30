## Client Version 1.12

### Wowm Representation
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
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `FORWARD` | 1 (0x01) |  |  |
| `BACKWARD` | 2 (0x02) |  |  |
| `STRAFE_LEFT` | 4 (0x04) |  |  |
| `STRAFE_RIGHT` | 8 (0x08) |  |  |
| `TURN_LEFT` | 16 (0x10) |  |  |
| `TURN_RIGHT` | 32 (0x20) |  |  |
| `PITCH_UP` | 64 (0x40) |  |  |
| `PITCH_DOWN` | 128 (0x80) |  |  |
| `WALK_MODE` | 256 (0x100) |  |  |
| `ON_TRANSPORT` | 512 (0x200) |  |  |
| `LEVITATING` | 1024 (0x400) |  |  |
| `FIXED_Z` | 2048 (0x800) |  |  |
| `ROOT` | 4096 (0x1000) |  |  |
| `JUMPING` | 8192 (0x2000) |  |  |
| `FALLINGFAR` | 16384 (0x4000) |  |  |
| `SWIMMING` | 2097152 (0x200000) |  |  |
| `SPLINE_ENABLED` | 4194304 (0x400000) |  |  |
| `CAN_FLY` | 8388608 (0x800000) |  |  |
| `FLYING` | 16777216 (0x1000000) |  |  |
| `ONTRANSPORT` | 33554432 (0x2000000) |  |  |
| `SPLINE_ELEVATION` | 67108864 (0x4000000) |  |  |
| `WATERWALKING` | 268435456 (0x10000000) |  |  |
| `SAFE_FALL` | 536870912 (0x20000000) |  |  |
| `HOVER` | 1073741824 (0x40000000) |  |  |
