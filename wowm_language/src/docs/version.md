# Version

## Protocol Version *

### Wowm Representation
```rust,ignore
struct Version {
    u8 major;
    u8 minor;
    u8 patch;
    u16 build;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 1 / - | u8 | major |  |  |
| 0x01 | 1 / - | u8 | minor |  |  |
| 0x02 | 1 / - | u8 | patch |  |  |
| 0x03 | 2 / Little | u16 | build |  |  |

