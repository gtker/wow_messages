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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| STAND | 0 | 0 | 0x0 |  |  |
| SIT | 1 | 1 | 0x1 |  |  |
| SIT_CHAIR | 2 | 2 | 0x2 |  |  |
| SLEEP | 3 | 3 | 0x3 |  |  |
| SIT_LOW_CHAIR | 4 | 4 | 0x4 |  |  |
| SIT_MEDIUM_CHAIR | 5 | 5 | 0x5 |  |  |
| SIT_HIGH_CHAIR | 6 | 6 | 0x6 |  |  |
| DEAD | 7 | 7 | 0x7 |  |  |
| KNEEL | 8 | 8 | 0x8 |  |  |
| CUSTOM | 9 | 9 | 0x9 |  | Used for Cthun according to cmangos. |
