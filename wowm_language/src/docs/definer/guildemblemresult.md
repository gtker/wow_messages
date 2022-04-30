## Client Version 1.12

## Wowm Representation
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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SUCCESS | 0 | 0 | 0x0 |  | Guild Emblem saved. |
| INVALID_TABARD_COLORS | 1 | 1 | 0x1 |  |  |
| NO_GUILD | 2 | 2 | 0x2 |  | vmangos: You are not part of a guild! |
| NOT_GUILD_MASTER | 3 | 3 | 0x3 |  | vmangos: Only guild leaders can create emblems. |
| NOT_ENOUGH_MONEY | 4 | 4 | 0x4 |  | vmangos: You can't afford to do that. |
| NO_MESSAGE | 5 | 5 | 0x5 |  | mangoszero: [This version] fails silently. |
