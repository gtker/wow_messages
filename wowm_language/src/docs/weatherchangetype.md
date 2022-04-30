## Client Version 1.12

### Wowm Representation
```rust,ignore
enum WeatherChangeType : u8 {
    SMOOTH = 0;
    INSTANT = 1;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SMOOTH` | 0 (0x00) |  |  |
| `INSTANT` | 1 (0x01) |  |  |
