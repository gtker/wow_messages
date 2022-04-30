## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg MSG_LIST_STABLED_PETS_Server = 0x026F {
    Guid npc;    
    u8 amount_of_pets;    
    u8 stable_slots;    
    StabledPet[amount_of_pets] pets;    
}

```
