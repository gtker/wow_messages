## Client Version 1.12

## Wowm Representation
```rust,ignore
enum FriendResult : u8 {
    DB_ERROR = 0x00;    
    LIST_FULL = 0x01;    
    ONLINE = 0x02;    
    OFFLINE = 0x03;    
    NOT_FOUND = 0x04;    
    REMOVED = 0x05;    
    ADDED_ONLINE = 0x06;    
    ADDED_OFFLINE = 0x07;    
    ALREADY = 0x08;    
    SELF = 0x09;    
    ENEMY = 0x0A;    
    IGNORE_FULL = 0x0B;    
    IGNORE_SELF = 0x0C;    
    IGNORE_NOT_FOUND = 0x0D;    
    IGNORE_ALREADY = 0x0E;    
    IGNORE_ADDED = 0x0F;    
    IGNORE_REMOVED = 0x10;    
    IGNORE_AMBIGUOUS = 0x11;    
    MUTE_FULL = 0x12;    
    MUTE_SELF = 0x13;    
    MUTE_NOT_FOUND = 0x14;    
    MUTE_ALREADY = 0x15;    
    MUTE_ADDED = 0x16;    
    MUTE_REMOVED = 0x17;    
    MUTE_AMBIGUOUS = 0x18;    
    UNKNOWN19 = 0x19;    
    UNKNOWN20 = 0x1A;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| DB_ERROR | 0x00 | 0 | 0x0 |  |  |
| LIST_FULL | 0x01 | 1 | 0x1 |  |  |
| ONLINE | 0x02 | 2 | 0x2 |  |  |
| OFFLINE | 0x03 | 3 | 0x3 |  |  |
| NOT_FOUND | 0x04 | 4 | 0x4 |  |  |
| REMOVED | 0x05 | 5 | 0x5 |  |  |
| ADDED_ONLINE | 0x06 | 6 | 0x6 |  |  |
| ADDED_OFFLINE | 0x07 | 7 | 0x7 |  |  |
| ALREADY | 0x08 | 8 | 0x8 |  |  |
| SELF | 0x09 | 9 | 0x9 |  |  |
| ENEMY | 0x0A | 10 | 0xA |  |  |
| IGNORE_FULL | 0x0B | 11 | 0xB |  |  |
| IGNORE_SELF | 0x0C | 12 | 0xC |  |  |
| IGNORE_NOT_FOUND | 0x0D | 13 | 0xD |  |  |
| IGNORE_ALREADY | 0x0E | 14 | 0xE |  |  |
| IGNORE_ADDED | 0x0F | 15 | 0xF |  |  |
| IGNORE_REMOVED | 0x10 | 16 | 0x10 |  |  |
| IGNORE_AMBIGUOUS | 0x11 | 17 | 0x11 |  |  |
| MUTE_FULL | 0x12 | 18 | 0x12 |  |  |
| MUTE_SELF | 0x13 | 19 | 0x13 |  |  |
| MUTE_NOT_FOUND | 0x14 | 20 | 0x14 |  |  |
| MUTE_ALREADY | 0x15 | 21 | 0x15 |  |  |
| MUTE_ADDED | 0x16 | 22 | 0x16 |  |  |
| MUTE_REMOVED | 0x17 | 23 | 0x17 |  |  |
| MUTE_AMBIGUOUS | 0x18 | 24 | 0x18 |  |  |
| UNKNOWN19 | 0x19 | 25 | 0x19 |  |  |
| UNKNOWN20 | 0x1A | 26 | 0x1A |  |  |
