## Protocol Version 2, Protocol Version 3

### Wowm Representation
```rust,ignore
enum RealmType : u32 {
    PLAYER_VS_ENVIRONMENT = 0;
    PLAYER_VS_PLAYER = 1;
    ROLEPLAYING = 6;
    ROLEPLAYING_PLAYER_VS_PLAYER = 8;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `PLAYER_VS_ENVIRONMENT` | 0 (0x00) |  |  |
| `PLAYER_VS_PLAYER` | 1 (0x01) |  |  |
| `ROLEPLAYING` | 6 (0x06) |  |  |
| `ROLEPLAYING_PLAYER_VS_PLAYER` | 8 (0x08) |  |  |
