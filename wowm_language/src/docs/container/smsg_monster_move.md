## Client Version 1.12

```rust,ignore
smsg SMSG_MONSTER_MOVE = 0xDD {
    PackedGuid guid;    
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    u32 spline_id;    
    MonsterMoveType move_type;    
}

```
