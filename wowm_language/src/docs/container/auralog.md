## Client Version 1.12

## Wowm Representation
```rust,ignore
struct AuraLog {
    AuraType aura_type;    
    if (aura_type == PERIODIC_DAMAGE        
        || aura_type == PERIODIC_DAMAGE_PERCENT) {        
        u32 damage1;        
        SpellSchool school;        
        u32 absorbed;        
        u32 resisted;        
    }    
    else if (aura_type == PERIODIC_HEAL        
        || aura_type == OBS_MOD_HEALTH) {        
        u32 damage2;        
    }    
    else if (aura_type == OBS_MOD_MANA        
        || aura_type == PERIODIC_ENERGIZE) {        
        u32 misc_value1;        
        u32 damage3;        
    }    
    else if (aura_type == PERIODIC_MANA_LEECH) {        
        u32 misc_value2;        
        u32 damage;        
        f32 gain_multiplier;        
    }    
}

```
