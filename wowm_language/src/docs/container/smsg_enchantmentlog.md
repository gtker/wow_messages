## Client Version 1.12

### Comment

cmangos and vmangos/mangoszero disagree about packed and extra u8

### Wowm Representation
```rust,ignore
smsg SMSG_ENCHANTMENTLOG = 0x01D7 {
    Guid target_guid;    
    Guid caster_guid;    
    u32 item;    
    u32 spell;    
    u8 unknown1;    
}

```
