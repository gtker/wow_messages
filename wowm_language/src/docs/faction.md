# Faction

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L3).
```rust,ignore
struct Faction {
    u32 reputation_list_id;
    u32 standing;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | reputation_list_id |  |  |
| 0x04 | 4 / Little | u32 | standing |  |  |

