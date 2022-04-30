## Client Version 1.12

### Wowm Representation
```rust,ignore
enum EnvironmentalDamageType : u32 {
    EXHAUSTED = 0;
    DROWNING = 1;
    FALL = 2;
    LAVA = 3;
    SLIME = 4;
    FIRE = 5;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `EXHAUSTED` | 0 (0x00) |  |  |
| `DROWNING` | 1 (0x01) |  |  |
| `FALL` | 2 (0x02) |  |  |
| `LAVA` | 3 (0x03) |  |  |
| `SLIME` | 4 (0x04) |  |  |
| `FIRE` | 5 (0x05) |  |  |
