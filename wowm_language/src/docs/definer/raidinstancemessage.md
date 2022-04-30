## Client Version 1.12

### Wowm Representation
```rust,ignore
enum RaidInstanceMessage : u32 {
    WARNING_HOURS = 1;
    WARNING_MIN = 2;
    WARNING_MIN_SOON = 3;
    WELCOME = 4;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `WARNING_HOURS` | 1 (0x01) |  | WARNING! %s is scheduled to reset in %d hour(s). |
| `WARNING_MIN` | 2 (0x02) |  | WARNING! %s is scheduled to reset in %d minute(s)! |
| `WARNING_MIN_SOON` | 3 (0x03) |  | WARNING! %s is scheduled to reset in %d minute(s). Please exit the zone or you will be returned to your bind location! |
| `WELCOME` | 4 (0x04) |  | Welcome to %s. This raid instance is scheduled to reset in %s. |
