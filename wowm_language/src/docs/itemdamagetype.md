## Client Version 1.12

### Wowm Representation
```rust,ignore
struct ItemDamageType {
    u32 damage_minimum;
    u32 damage_maximum;
    u32 damage_type;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | damage_minimum |  |  |
| 0x04 | 4 / Little | u32 | damage_maximum |  |  |
| 0x08 | 4 / Little | u32 | damage_type |  | mangoszero/vmangos/cmangos: id from Resistances.dbc |

