## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_ATTACKERSTATEUPDATE = 0x014A {
    u32 hit_info;    
    PackedGuid attacker;    
    PackedGuid target;    
    u32 total_damage;    
}

```
