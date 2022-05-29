# SpellMiss

## Client Version 1.12

### Wowm Representation
```rust,ignore
struct SpellMiss {
    Guid target_guid;
    SpellMissInfo miss_info;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | target_guid |  |  |
| 0x08 | ? / - | [SpellMissInfo](spellmissinfo.md) | miss_info |  |  |

