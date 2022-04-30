## Client Version 1.12

```rust,ignore
smsg SMSG_SPELL_START = 0x131 {
    PackedGuid cast_item;    
    PackedGuid caster;    
    u32 spell;    
    CastFlags flags;    
    u32 timer;    
    SpellCastTargets targets;    
    if (flags & AMMO) {        
        u32 ammo_display_id;        
        u32 ammo_inventory_type;        
    }    
}

```
