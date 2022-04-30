## Client Version 1.12

### Wowm Representation
```rust,ignore
enum PetitionTurnInResult : u32 {
    OK = 0;
    ALREADY_SIGNED = 1;
    ALREADY_IN_GUILD = 2;
    CANT_SIGN_OWN = 3;
    NEED_MORE = 4;
    NOT_SERVER = 5;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `OK` | 0 (0x00) |  |  |
| `ALREADY_SIGNED` | 1 (0x01) |  |  |
| `ALREADY_IN_GUILD` | 2 (0x02) |  |  |
| `CANT_SIGN_OWN` | 3 (0x03) |  |  |
| `NEED_MORE` | 4 (0x04) |  |  |
| `NOT_SERVER` | 5 (0x05) |  |  |
