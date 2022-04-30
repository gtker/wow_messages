## Client Version 1.12

```rust,ignore
struct SpellLog {
    SpellEffect effect;    
    u32 amount_of_logs = 1;    
    if (effect == POWER_DRAIN) {        
        Guid target1;        
        u32 unknown1;        
        u32 unknown2;        
        f32 unknown3;        
    }    
    else if (effect == ADD_EXTRA_ATTACKS) {        
        Guid target2;        
        u32 unknown4;        
    }    
    else if (effect == INTERRUPT_CAST) {        
        Guid target3;        
        u32 interrupted_spell;        
    }    
    else if (effect == DURABILITY_DAMAGE) {        
        Guid target4;        
        u32 unknown5;        
        u32 unknown6;        
    }    
    else if (effect == CREATE_ITEM) {        
        u32 spell_effect_item_type;        
    }    
    else if (effect == FEED_PET) {        
        u32 item_target_entry;        
    }    
    else if (effect == RESURRECT        
        || effect == DISPEL        
        || effect == THREAT        
        || effect == DISTRACT        
        || effect == SANCTUARY        
        || effect == THREAT_ALL        
        || effect == DISPEL_MECHANIC        
        || effect == RESURRECT_NEW        
        || effect == ATTACK_ME        
        || effect == SKIN_PLAYER_CORPSE        
        || effect == MODIFY_THREAT_PERCENT        
        || effect == UNKNOWN126        
        || effect == DISMISS_PET        
        || effect == OPEN_LOCK        
        || effect == OPEN_LOCK_ITEM        
        || effect == INSTAKILL) {        
        Guid target5;        
    }    
}

```
