## Client Version 1.12

## Wowm Representation
```rust,ignore
enum TradeStatus : u32 {
    BUSY = 0;    
    BEGIN_TRADE = 1;    
    OPEN_WINDOW = 2;    
    TRADE_CANCELED = 3;    
    TRADE_ACCEPT = 4;    
    BUSY_2 = 5;    
    NO_TARGET = 6;    
    BACK_TO_TRADE = 7;    
    TRADE_COMPLETE = 8;    
    TRADE_REJECTED = 9;    
    TARGET_TO_FAR = 10;    
    WRONG_FACTION = 11;    
    CLOSE_WINDOW = 12;    
    UNKNOWN_13 = 13;    
    IGNORE_YOU = 14;    
    YOU_STUNNED = 15;    
    TARGET_STUNNED = 16;    
    YOU_DEAD = 17;    
    TARGET_DEAD = 18;    
    YOU_LOGOUT = 19;    
    TARGET_LOGOUT = 20;    
    TRIAL_ACCOUNT = 21;    
    ONLY_CONJURED = 22;    
    NOT_ON_TAPLIST = 23;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| BUSY | 0 | 0 | 0x0 |  |  |
| BEGIN_TRADE | 1 | 1 | 0x1 |  |  |
| OPEN_WINDOW | 2 | 2 | 0x2 |  |  |
| TRADE_CANCELED | 3 | 3 | 0x3 |  |  |
| TRADE_ACCEPT | 4 | 4 | 0x4 |  |  |
| BUSY_2 | 5 | 5 | 0x5 |  |  |
| NO_TARGET | 6 | 6 | 0x6 |  |  |
| BACK_TO_TRADE | 7 | 7 | 0x7 |  |  |
| TRADE_COMPLETE | 8 | 8 | 0x8 |  |  |
| TRADE_REJECTED | 9 | 9 | 0x9 |  |  |
| TARGET_TO_FAR | 10 | 10 | 0xA |  |  |
| WRONG_FACTION | 11 | 11 | 0xB |  |  |
| CLOSE_WINDOW | 12 | 12 | 0xC |  |  |
| UNKNOWN_13 | 13 | 13 | 0xD |  |  |
| IGNORE_YOU | 14 | 14 | 0xE |  |  |
| YOU_STUNNED | 15 | 15 | 0xF |  |  |
| TARGET_STUNNED | 16 | 16 | 0x10 |  |  |
| YOU_DEAD | 17 | 17 | 0x11 |  |  |
| TARGET_DEAD | 18 | 18 | 0x12 |  |  |
| YOU_LOGOUT | 19 | 19 | 0x13 |  |  |
| TARGET_LOGOUT | 20 | 20 | 0x14 |  |  |
| TRIAL_ACCOUNT | 21 | 21 | 0x15 |  |  |
| ONLY_CONJURED | 22 | 22 | 0x16 |  |  |
| NOT_ON_TAPLIST | 23 | 23 | 0x17 |  |  |
