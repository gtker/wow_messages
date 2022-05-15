## Client Version 1.12

### Wowm Representation
```rust,ignore
struct PetSpellCooldown {
    u16 spell_id;
    u16 spell_category;
    u32 cooldown_in_msecs;
    u32 category_cooldown_in_msecs;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 2 / Little | u16 | spell_id |  |  |
| 0x02 | 2 / Little | u16 | spell_category |  | mangoszero: sets to 0 |
| 0x04 | 4 / Little | u32 | cooldown_in_msecs |  |  |
| 0x08 | 4 / Little | u32 | category_cooldown_in_msecs |  |  |
