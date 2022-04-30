## Client Version 1.12

```rust,ignore
smsg SMSG_SPELL_GO = 0x0132 {
    PackedGuid cast_item;    
    PackedGuid caster;    
    u32 spell;    
    CastFlags flags;    
    u8 amount_of_hits;    
    Guid[amount_of_hits] hits;    
    u8 amount_of_misses;    
    SpellMiss[amount_of_misses] misses;    
    SpellCastTargets targets;    
    if (flags & AMMO) {        
        u32 ammo_display_id;        
        u32 ammo_inventory_type;        
    }    
}

```
