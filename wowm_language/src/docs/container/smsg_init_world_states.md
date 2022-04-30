## Client Version 1.12

```rust,ignore
smsg SMSG_INIT_WORLD_STATES = 0x02C2 {
    Map map;    
    Area area;    
    u16 amount_of_states;    
    WorldState[amount_of_states] states;    
}

```
