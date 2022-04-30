## Client Version 1.12

## Wowm Representation
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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| CREATE | 0x00 | 0 | 0x0 |  |  |
| INVITE | 0x01 | 1 | 0x1 |  |  |
| QUIT | 0x03 | 3 | 0x3 |  |  |
| FOUNDER | 0x0E | 14 | 0xE |  |  |
| UNKNOWN19 | 0x13 | 19 | 0x13 |  | cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE |
| UNKNOWN20 | 0x14 | 20 | 0x14 |  | cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE |
