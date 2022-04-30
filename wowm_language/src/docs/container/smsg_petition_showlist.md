## Client Version 1.12

```rust,ignore
smsg SMSG_PETITION_SHOWLIST = 0x01BC {
    Guid npc;    
    u8 amount_of_petitions;    
    PetitionShowlist[amount_of_petitions] petitions;    
}

```
