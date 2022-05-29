## Client Version 1.12

### Wowm Representation
```rust,ignore
struct ForcedReaction {
    u32 faction_id;
    u32 reputation_rank;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | faction_id |  | vmangos: faction_id (Faction.dbc) |
| 0x04 | 4 / Little | u32 | reputation_rank |  | vmangos: reputation rank |

