## Client Version 1.12

## Wowm Representation
```rust,ignore
enum UnitStandState : u8 {
    STAND = 0;    
    SIT = 1;    
    SIT_CHAIR = 2;    
    SLEEP = 3;    
    SIT_LOW_CHAIR = 4;    
    SIT_MEDIUM_CHAIR = 5;    
    SIT_HIGH_CHAIR = 6;    
    DEAD = 7;    
    KNEEL = 8;    
    CUSTOM = 9;    
}

```
## Type
The basic type is `u8`, a 1 byte (8 bit) integer.
## Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `STAND` | 0 (0x00) |  |  |
| `SIT` | 1 (0x01) |  |  |
| `SIT_CHAIR` | 2 (0x02) |  |  |
| `SLEEP` | 3 (0x03) |  |  |
| `SIT_LOW_CHAIR` | 4 (0x04) |  |  |
| `SIT_MEDIUM_CHAIR` | 5 (0x05) |  |  |
| `SIT_HIGH_CHAIR` | 6 (0x06) |  |  |
| `DEAD` | 7 (0x07) |  |  |
| `KNEEL` | 8 (0x08) |  |  |
| `CUSTOM` | 9 (0x09) |  | Used for Cthun according to cmangos. |
