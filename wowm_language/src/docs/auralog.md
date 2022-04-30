## Client Version 1.12

### Wowm Representation
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
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [AuraType](auratype.md) | aura_type |  |

If aura_type is equal to `PERIODIC_DAMAGE` **or** 
is equal to `PERIODIC_DAMAGE_PERCENT`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | damage1 |  |
| - | ? / - | [SpellSchool](spellschool.md) | school |  |
| - | 4 / Little | u32 | absorbed |  |
| - | 4 / Little | u32 | resisted |  |

Else If aura_type is equal to `PERIODIC_HEAL` **or** 
is equal to `OBS_MOD_HEALTH`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | damage2 |  |

Else If aura_type is equal to `OBS_MOD_MANA` **or** 
is equal to `PERIODIC_ENERGIZE`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | misc_value1 |  |
| - | 4 / Little | u32 | damage3 |  |

Else If aura_type is equal to `PERIODIC_MANA_LEECH`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | misc_value2 |  |
| - | 4 / Little | u32 | damage |  |
| - | 4 / Little | f32 | gain_multiplier |  |
