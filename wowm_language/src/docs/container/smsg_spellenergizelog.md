## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_SPELLENERGIZELOG = 0x0151 {
    PackedGuid victim_guid;    
    PackedGuid caster_guid;    
    u32 spell;    
    PowerType power;    
    u32 damage;    
}

```
