## Client Version 1.12

### Wowm Representation
```rust,ignore
struct ItemSpells {
    u32 spell;
    u32 spell_trigger;
    u32 spell_charges;
    u32 spell_cooldown;
    u32 spell_category;
    u32 spell_category_cooldown;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | spell |  |
| 0x04 | 4 / Little | u32 | spell_trigger |  |
| 0x08 | 4 / Little | u32 | spell_charges |  |
| 0x0C | 4 / Little | u32 | spell_cooldown |  |
| 0x10 | 4 / Little | u32 | spell_category |  |
| 0x14 | 4 / Little | u32 | spell_category_cooldown |  |
