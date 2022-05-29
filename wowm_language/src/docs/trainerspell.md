## Client Version 1.12

### Wowm Representation
```rust,ignore
struct TrainerSpell {
    u32 spell;
    TrainerSpellState state;
    u32 spell_cost;
    u32 talent_point_cost;
    u32 first_rank;
    u8 required_level;
    u32 required_skill;
    u32 required_skill_value;
    u32 spell_chain_required;
    u32 spell_chain_previous;
    u32 unknown1;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | spell |  | cmangos: learned spell (or cast-spell in profession case) |
| 0x04 | ? / - | [TrainerSpellState](trainerspellstate.md) | state |  |  |
| - | 4 / Little | u32 | spell_cost |  |  |
| - | 4 / Little | u32 | talent_point_cost |  | cmangos: spells don't cost talent points<br/>cmangos: set to 0 |
| - | 4 / Little | u32 | first_rank |  | cmangos: must be equal prev. field to have learn button in enabled state<br/>cmangos: 1 for true 0 for false |
| - | 1 / - | u8 | required_level |  |  |
| - | 4 / Little | u32 | required_skill |  |  |
| - | 4 / Little | u32 | required_skill_value |  |  |
| - | 4 / Little | u32 | spell_chain_required |  |  |
| - | 4 / Little | u32 | spell_chain_previous |  |  |
| - | 4 / Little | u32 | unknown1 |  | cmangos/vmangos/mangoszero: all set 0 |

