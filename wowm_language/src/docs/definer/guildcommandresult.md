## Client Version 1.12

## Wowm Representation
```rust,ignore
enum GuildCommandResult : u8 {
    PLAYER_NO_MORE_IN_GUILD = 0x00;    
    GUILD_INTERNAL = 0x01;    
    ALREADY_IN_GUILD = 0x02;    
    ALREADY_IN_GUILD_S = 0x03;    
    INVITED_TO_GUILD = 0x04;    
    ALREADY_INVITED_TO_GUILD_S = 0x05;    
    GUILD_NAME_INVALID = 0x06;    
    GUILD_NAME_EXISTS_S = 0x07;    
    GUILD_LEADER_LEAVE = 0x08;    
    GUILD_PERMISSIONS = 0x08;    
    GUILD_PLAYER_NOT_IN_GUILD = 0x09;    
    GUILD_PLAYER_NOT_IN_GUILD_S = 0x0A;    
    GUILD_PLAYER_NOT_FOUND_S = 0x0B;    
    GUILD_NOT_ALLIED = 0x0C;    
    GUILD_RANK_TOO_HIGH_S = 0x0D;    
    GUILD_RANK_TOO_LOW_S = 0x0E;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| PLAYER_NO_MORE_IN_GUILD | 0x00 | 0 | 0x0 |  |  |
| GUILD_INTERNAL | 0x01 | 1 | 0x1 |  |  |
| ALREADY_IN_GUILD | 0x02 | 2 | 0x2 |  |  |
| ALREADY_IN_GUILD_S | 0x03 | 3 | 0x3 |  |  |
| INVITED_TO_GUILD | 0x04 | 4 | 0x4 |  |  |
| ALREADY_INVITED_TO_GUILD_S | 0x05 | 5 | 0x5 |  |  |
| GUILD_NAME_INVALID | 0x06 | 6 | 0x6 |  |  |
| GUILD_NAME_EXISTS_S | 0x07 | 7 | 0x7 |  |  |
| GUILD_LEADER_LEAVE | 0x08 | 8 | 0x8 |  |  |
| GUILD_PERMISSIONS | 0x08 | 8 | 0x8 |  |  |
| GUILD_PLAYER_NOT_IN_GUILD | 0x09 | 9 | 0x9 |  |  |
| GUILD_PLAYER_NOT_IN_GUILD_S | 0x0A | 10 | 0xA |  |  |
| GUILD_PLAYER_NOT_FOUND_S | 0x0B | 11 | 0xB |  |  |
| GUILD_NOT_ALLIED | 0x0C | 12 | 0xC |  |  |
| GUILD_RANK_TOO_HIGH_S | 0x0D | 13 | 0xD |  |  |
| GUILD_RANK_TOO_LOW_S | 0x0E | 14 | 0xE |  |  |
