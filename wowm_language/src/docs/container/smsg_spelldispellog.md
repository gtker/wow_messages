## Client Version 1.12

```rust,ignore
smsg SMSG_SPELLDISPELLOG = 0x27B {
    Guid victim;    
    Guid caster;    
    u32 amount_of_spells;    
    u32[amount_of_spells] spells;    
}

```
