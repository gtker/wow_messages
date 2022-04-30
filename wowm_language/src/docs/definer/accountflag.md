## Protocol Version 8

### Wowm Representation
```rust,ignore
flag AccountFlag : u32 {
    GM = 0x000001;
    TRIAL = 0x000008;
    PROPASS = 0x800000;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `GM` | 1 (0x01) |  |  |
| `TRIAL` | 8 (0x08) |  |  |
| `PROPASS` | 8388608 (0x800000) |  |  |
