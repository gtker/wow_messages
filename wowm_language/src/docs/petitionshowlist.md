# PetitionShowlist
## Client Version 1.12

### Wowm Representation
```rust,ignore
struct PetitionShowlist {
    u32 index;
    u32 charter_entry = 5863;
    u32 charter_display_id = 16161;
    u32 guild_charter_cost;
    u32 unknown1;
    u32 unknown2;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | index |  |
| 0x04 | 4 / Little | u32 | charter_entry |  |
| 0x08 | 4 / Little | u32 | charter_display_id |  |
| 0x0C | 4 / Little | u32 | guild_charter_cost |  |
| 0x10 | 4 / Little | u32 | unknown1 |  |
| 0x14 | 4 / Little | u32 | unknown2 |  |
