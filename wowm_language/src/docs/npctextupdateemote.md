# NpcTextUpdateEmote
## Client Version 1.12

### Wowm Representation
```rust,ignore
struct NpcTextUpdateEmote {
    u32 delay;
    u32 emote;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | delay |  |
| 0x04 | 4 / Little | u32 | emote |  |
