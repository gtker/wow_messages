## Client Version 1.12

### Wowm Representation
```rust,ignore
struct InitialSpell {
    u16 spell_id;
    u16 unknown1;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 2 / Little | u16 | spell_id |  | cmangos/mangoszero: only send 'first' part of spell |
| 0x02 | 2 / Little | u16 | unknown1 |  | cmangos/mangoszero: sets to 0<br/>cmangos/mangoszero: it's not slot id |
