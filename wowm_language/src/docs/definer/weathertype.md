## Client Version 1.12

### Wowm Representation
```rust,ignore
enum WeatherType : u32 {
    FINE = 0;    
    RAIN = 1;    
    SNOW = 2;    
    STORM = 3;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `FINE` | 0 (0x00) |  |  |
| `RAIN` | 1 (0x01) |  |  |
| `SNOW` | 2 (0x02) |  |  |
| `STORM` | 3 (0x03) |  |  |
