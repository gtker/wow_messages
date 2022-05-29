# CooldownSpell

## Client Version 1.12

### Wowm Representation
```rust,ignore
struct CooldownSpell {
    u16 spell_id;
    u16 item_id;
    u16 spell_category;
    u32 cooldown_in_msecs;
    u32 category_cooldown_in_msecs;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 2 / Little | u16 | spell_id |  |  |
| 0x02 | 2 / Little | u16 | item_id |  | cmangos/mangoszero: cast item id |
| 0x04 | 2 / Little | u16 | spell_category |  |  |
| 0x06 | 4 / Little | u32 | cooldown_in_msecs |  |  |
| 0x0A | 4 / Little | u32 | category_cooldown_in_msecs |  |  |

