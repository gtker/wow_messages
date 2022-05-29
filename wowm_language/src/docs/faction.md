# Faction

## Client Version 1.12

### Wowm Representation
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

