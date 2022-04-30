## Client Version 1.12

```rust,ignore
smsg SMSG_SPELLLOGMISS = 0x24B {
    u32 id;    
    Guid caster_guid;    
    u8 unknown1;    
    u32 amount_of_targets;    
    SpellMiss[amount_of_targets] targets;    
}

```
