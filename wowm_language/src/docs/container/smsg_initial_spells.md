## Client Version 1.12

```rust,ignore
smsg SMSG_INITIAL_SPELLS = 0x12A {
    u8 unknown1;    
    u16 spell_count;    
    InitialSpell[spell_count] initial_spells;    
    u16 cooldown_count;    
    CooldownSpell[cooldown_count] cooldowns;    
}

```
