## Client Version 1.12

```rust,ignore
smsg SMSG_ATTACKERSTATEUPDATE = 0x14A {
    u32 hit_info;    
    PackedGuid attacker;    
    PackedGuid target;    
    u32 total_damage;    
}

```
