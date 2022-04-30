## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_SPELLLOGEXECUTE = 0x024C {
    PackedGuid caster;    
    u32 spell;    
    u32 amount_of_effects;    
    SpellLog[amount_of_effects] logs;    
}

```
