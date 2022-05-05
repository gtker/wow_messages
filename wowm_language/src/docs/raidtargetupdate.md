# RaidTargetUpdate
## Client Version 1.12

### Wowm Representation
```rust,ignore
struct RaidTargetUpdate {
    RaidTargetIndex index;
    Guid guid;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [RaidTargetIndex](raidtargetindex.md) | index |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
