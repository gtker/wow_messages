## Client Version 1.12

### Wowm Representation
```rust,ignore
enum GuildCommand : u8 {
    CREATE = 0x00;
    INVITE = 0x01;
    QUIT = 0x03;
    FOUNDER = 0x0E;
    UNKNOWN19 = 0x13;
    UNKNOWN20 = 0x14;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `CREATE` | 0 (0x00) |  |  |
| `INVITE` | 1 (0x01) |  |  |
| `QUIT` | 3 (0x03) |  |  |
| `FOUNDER` | 14 (0x0E) |  |  |
| `UNKNOWN19` | 19 (0x13) |  | cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE |
| `UNKNOWN20` | 20 (0x14) |  | cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE |
