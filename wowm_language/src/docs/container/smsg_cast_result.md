## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_CAST_RESULT = 0x0130 {
    u32 spell;    
    SimpleSpellCastResult result;    
    if (result != FAILURE) {        
        CastFailureReason reason;        
        if (reason == REQUIRES_SPELL_FOCUS) {            
            u32 required_spell_focus;            
        }        
        else if (reason == REQUIRES_AREA) {            
            Area area;            
        }        
        else if (reason == EQUIPPED_ITEM_CLASS) {            
            u32 equipped_item_class;            
            u32 equipped_item_subclass_mask;            
            u32 equipped_item_inventory_type_mask;            
        }        
    }    
}

```
