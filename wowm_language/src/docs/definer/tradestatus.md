## Client Version 1.12

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
