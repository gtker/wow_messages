## Client Version 1.12

### Wowm Representation
```rust,ignore
enum Language : u32 {
    UNIVERSAL = 0;
    ORCISH = 1;
    DARNASSIAN = 2;
    TAURAHE = 3;
    DWARVISH = 6;
    COMMON = 7;
    DEMONIC = 8;
    TITAN = 9;
    THALASSIAN = 10;
    DRACONIC = 11;
    KALIMAG = 12;
    GNOMISH = 13;
    TROLL = 14;
    GUTTERSPEAK = 33;
    ADDON = 0xFFFFFFFF;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `UNIVERSAL` | 0 (0x00) |  |  |
| `ORCISH` | 1 (0x01) |  |  |
| `DARNASSIAN` | 2 (0x02) |  |  |
| `TAURAHE` | 3 (0x03) |  |  |
| `DWARVISH` | 6 (0x06) |  |  |
| `COMMON` | 7 (0x07) |  |  |
| `DEMONIC` | 8 (0x08) |  |  |
| `TITAN` | 9 (0x09) |  |  |
| `THALASSIAN` | 10 (0x0A) |  |  |
| `DRACONIC` | 11 (0x0B) |  |  |
| `KALIMAG` | 12 (0x0C) |  |  |
| `GNOMISH` | 13 (0x0D) |  |  |
| `TROLL` | 14 (0x0E) |  |  |
| `GUTTERSPEAK` | 33 (0x21) |  |  |
| `ADDON` | 4294967295 (0xFFFFFFFF) |  |  |
