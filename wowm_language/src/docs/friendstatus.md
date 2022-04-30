## Client Version 1.12

### Wowm Representation
```rust,ignore
enum FriendStatus : u8 {
    OFFLINE = 0;
    ONLINE = 1;
    AFK = 2;
    UNKNOWN3 = 3;
    DND = 4;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `OFFLINE` | 0 (0x00) |  |  |
| `ONLINE` | 1 (0x01) |  |  |
| `AFK` | 2 (0x02) |  |  |
| `UNKNOWN3` | 3 (0x03) |  |  |
| `DND` | 4 (0x04) |  |  |
