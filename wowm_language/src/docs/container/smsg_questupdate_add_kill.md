## Client Version 1.12

```rust,ignore
smsg SMSG_QUESTUPDATE_ADD_KILL = 0x0199 {
    u32 quest_id;    
    u32 create_id;    
    u32 kill_count;    
    u32 required_kill_count;    
    Guid guid;    
}

```
