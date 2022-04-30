## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_SPELLDISPELLOG = 0x027B {
    Guid victim;    
    Guid caster;    
    u32 amount_of_spells;    
    u32[amount_of_spells] spells;    
}

```
