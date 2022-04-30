## Client Version 1.12

```rust,ignore
smsg SMSG_TRAINER_LIST = 0x01B1 {
    Guid guid;    
    u32 trainer_type;    
    u32 amount_of_spells;    
    TrainerSpell[amount_of_spells] spells;    
    CString greeting;    
}

```
