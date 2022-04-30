## Client Version 1.12

```rust,ignore
smsg SMSG_PET_SPELLS = 0x179 {
    Guid pet;    
    u32 unknown1;    
    PetReactState react;    
    PetCommandState command;    
    u16 unknown2;    
    u32[10] action_bars;    
    u8 amount_of_spells;    
    u32[amount_of_spells] spells;    
    u8 amount_of_cooldowns;    
    PetSpellCooldown[amount_of_cooldowns] cooldowns;    
}

```
