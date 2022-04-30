## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_PERIODICAURALOG = 0x024E {
    PackedGuid target;    
    PackedGuid caster;    
    u32 spell;    
    u32 amount_of_auras;    
    AuraLog[amount_of_auras] auras;    
}

```
