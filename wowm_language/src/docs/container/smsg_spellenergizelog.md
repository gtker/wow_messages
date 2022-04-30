## Client Version 1.12

```rust,ignore
smsg SMSG_SPELLENERGIZELOG = 0x151 {
    PackedGuid victim_guid;    
    PackedGuid caster_guid;    
    u32 spell;    
    PowerType power;    
    u32 damage;    
}

```
