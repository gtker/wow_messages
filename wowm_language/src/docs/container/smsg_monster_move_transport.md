## Client Version 1.12

```rust,ignore
smsg SMSG_MONSTER_MOVE_TRANSPORT = 0x2AE {
    PackedGuid transport;    
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    u32 spline_id;    
    MonsterMoveType move_type;    
}

```
