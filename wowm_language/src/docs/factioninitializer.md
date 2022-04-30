## Client Version 1.12

### Wowm Representation
```rust,ignore
struct FactionInitializer {
    FactionFlag flag;
    u32 standing;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [FactionFlag](factionflag.md) | flag |  |
| - | 4 / Little | u32 | standing |  |
