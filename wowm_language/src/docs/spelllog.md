## Client Version 1.12

### Wowm Representation
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
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | ? / - | [SpellEffect](spelleffect.md) | effect |  |  |
| - | 4 / Little | u32 | amount_of_logs |  | vmangos/cmangos/mangoszero: Can be variable but all use constant 1 |

If effect is equal to `POWER_DRAIN`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | target1 |  |  |
| - | 4 / Little | u32 | unknown1 |  |  |
| - | 4 / Little | u32 | unknown2 |  |  |
| - | 4 / Little | f32 | unknown3 |  |  |

Else If effect is equal to `ADD_EXTRA_ATTACKS`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | target2 |  |  |
| - | 4 / Little | u32 | unknown4 |  |  |

Else If effect is equal to `INTERRUPT_CAST`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | target3 |  |  |
| - | 4 / Little | u32 | interrupted_spell |  |  |

Else If effect is equal to `DURABILITY_DAMAGE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | target4 |  |  |
| - | 4 / Little | u32 | unknown5 |  |  |
| - | 4 / Little | u32 | unknown6 |  |  |

Else If effect is equal to `CREATE_ITEM`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | spell_effect_item_type |  |  |

Else If effect is equal to `FEED_PET`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | item_target_entry |  |  |

Else If effect is equal to `RESURRECT` **or** 
is equal to `DISPEL` **or** 
is equal to `THREAT` **or** 
is equal to `DISTRACT` **or** 
is equal to `SANCTUARY` **or** 
is equal to `THREAT_ALL` **or** 
is equal to `DISPEL_MECHANIC` **or** 
is equal to `RESURRECT_NEW` **or** 
is equal to `ATTACK_ME` **or** 
is equal to `SKIN_PLAYER_CORPSE` **or** 
is equal to `MODIFY_THREAT_PERCENT` **or** 
is equal to `UNKNOWN126` **or** 
is equal to `DISMISS_PET` **or** 
is equal to `OPEN_LOCK` **or** 
is equal to `OPEN_LOCK_ITEM` **or** 
is equal to `INSTAKILL`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | target5 |  |  |

