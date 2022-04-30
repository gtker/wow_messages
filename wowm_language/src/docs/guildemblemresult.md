## Client Version 1.12

### Wowm Representation
```rust,ignore
enum GuildEmblemResult : u32 {
    SUCCESS = 0;
    INVALID_TABARD_COLORS = 1;
    NO_GUILD = 2;
    NOT_GUILD_MASTER = 3;
    NOT_ENOUGH_MONEY = 4;
    NO_MESSAGE = 5;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SUCCESS` | 0 (0x00) |  | Guild Emblem saved. |
| `INVALID_TABARD_COLORS` | 1 (0x01) |  |  |
| `NO_GUILD` | 2 (0x02) |  | vmangos: You are not part of a guild! |
| `NOT_GUILD_MASTER` | 3 (0x03) |  | vmangos: Only guild leaders can create emblems. |
| `NOT_ENOUGH_MONEY` | 4 (0x04) |  | vmangos: You can't afford to do that. |
| `NO_MESSAGE` | 5 (0x05) |  | mangoszero: [This version] fails silently. |
