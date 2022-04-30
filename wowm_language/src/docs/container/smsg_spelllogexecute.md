## Client Version 1.12

```rust,ignore
smsg SMSG_SPELLLOGEXECUTE = 0x24C {
    PackedGuid caster;    
    u32 spell;    
    u32 amount_of_effects;    
    SpellLog[amount_of_effects] logs;    
}

```
