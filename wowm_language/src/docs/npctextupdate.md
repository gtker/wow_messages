## Client Version 1.12

### Wowm Representation
```rust,ignore
struct NpcTextUpdate {
    f32 probability;
    CString[2] texts;
    Language language;
    NpcTextUpdateEmote[3] emotes;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | f32 | probability |  |
| 0x04 | ? / - | CString[2] | texts |  |
| - | ? / - | [Language](language.md) | language |  |
| - | ? / - | [NpcTextUpdateEmote](npctextupdateemote.md)[3] | emotes |  |
