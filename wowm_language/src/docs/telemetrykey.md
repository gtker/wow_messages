# TelemetryKey
## Protocol Version 2, Protocol Version 3, Protocol Version 8

### Wowm Representation
```rust,ignore
struct TelemetryKey {
    u16 unknown1;
    u32 unknown2;
    u8[4] unknown3;
    u8[20] unknown4;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 2 / Little | u16 | unknown1 |  |
| 0x02 | 4 / Little | u32 | unknown2 |  |
| 0x06 | ? / - | u8[4] | unknown3 |  |
| - | ? / - | u8[20] | unknown4 |  |
