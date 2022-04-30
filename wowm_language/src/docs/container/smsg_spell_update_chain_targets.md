## Client Version 1.12

```rust,ignore
smsg SMSG_SPELL_UPDATE_CHAIN_TARGETS = 0x0330 {
    Guid caster;    
    u32 spell;    
    u32 amount_of_targets;    
    Guid[amount_of_targets] targets;    
}

```
